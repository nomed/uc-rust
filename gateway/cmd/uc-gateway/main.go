// Command uc-gateway exposes the generated REST/JSON adapter for the canonical
// Rust/Tonic RuntimeService. It owns process wiring only; all business behavior
// remains behind the gRPC contract.
package main

import (
	"context"
	"errors"
	"flag"
	"log/slog"
	"net"
	"net/http"
	"os"
	"os/signal"
	"syscall"
	"time"

	"github.com/grpc-ecosystem/grpc-gateway/v2/runtime"
	runtimev1 "github.com/nomed/uc-rust/gateway/gen/go/uc/runtime/v1"
	"github.com/nomed/uc-rust/gateway/internal/docs"
	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials/insecure"
)

const (
	shutdownTimeout = 10 * time.Second
	readinessTimeout = 500 * time.Millisecond
)

func main() {
	if err := run(); err != nil {
		slog.Error("gateway stopped", "error", err)
		os.Exit(1)
	}
}

func run() error {
	listenAddr := flag.String("listen", envOrDefault("UC_GATEWAY_ADDR", "0.0.0.0:8080"), "REST listen address")
	grpcEndpoint := flag.String("grpc-endpoint", envOrDefault("UC_GRPC_ENDPOINT", "127.0.0.1:50051"), "Rust/Tonic gRPC endpoint")
	flag.Parse()

	ctx, stop := signal.NotifyContext(context.Background(), os.Interrupt, syscall.SIGTERM)
	defer stop()

	gateway := runtime.NewServeMux(
		runtime.WithIncomingHeaderMatcher(forwardedHeaderMatcher),
	)
	if err := runtimev1.RegisterRuntimeServiceHandlerFromEndpoint(
		ctx,
		gateway,
		*grpcEndpoint,
		[]grpc.DialOption{grpc.WithTransportCredentials(insecure.NewCredentials())},
	); err != nil {
		return err
	}

	mux := http.NewServeMux()
	mux.Handle("/", gateway)
	mux.HandleFunc("GET /healthz", func(w http.ResponseWriter, _ *http.Request) {
		w.WriteHeader(http.StatusOK)
		_, _ = w.Write([]byte("ok\n"))
	})
	mux.HandleFunc("GET /readyz", readinessHandler(*grpcEndpoint))
	docs.Register(mux)

	server := &http.Server{
		Addr:              *listenAddr,
		Handler:           mux,
		ReadHeaderTimeout: 5 * time.Second,
	}

	errCh := make(chan error, 1)
	go func() {
		slog.Info("REST gateway listening", "address", *listenAddr, "grpc_endpoint", *grpcEndpoint)
		errCh <- server.ListenAndServe()
	}()

	select {
	case <-ctx.Done():
		shutdownCtx, cancel := context.WithTimeout(context.Background(), shutdownTimeout)
		defer cancel()
		return server.Shutdown(shutdownCtx)
	case err := <-errCh:
		if errors.Is(err, http.ErrServerClosed) {
			return nil
		}
		return err
	}
}

func readinessHandler(grpcEndpoint string) http.HandlerFunc {
	return func(w http.ResponseWriter, _ *http.Request) {
		connection, err := net.DialTimeout("tcp", grpcEndpoint, readinessTimeout)
		if err != nil {
			http.Error(w, "gRPC backend unavailable", http.StatusServiceUnavailable)
			return
		}
		_ = connection.Close()
		w.WriteHeader(http.StatusOK)
		_, _ = w.Write([]byte("ready\n"))
	}
}

func envOrDefault(name, fallback string) string {
	if value := os.Getenv(name); value != "" {
		return value
	}
	return fallback
}

func forwardedHeaderMatcher(header string) (string, bool) {
	switch http.CanonicalHeaderKey(header) {
	case "Traceparent", "Tracestate", "X-Correlation-Id", "X-Timeout-Ms", "X-Uc-Cancelled":
		return header, true
	default:
		return runtime.DefaultHeaderMatcher(header)
	}
}
