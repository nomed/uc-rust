package main

import (
	"io"
	"net/http"
	"net/http/httptest"
	"strings"
	"testing"
)

func TestEnvOrDefault(t *testing.T) {
	t.Setenv("UC_TEST_VALUE", "configured")
	if got := envOrDefault("UC_TEST_VALUE", "fallback"); got != "configured" {
		t.Fatalf("expected configured value, got %q", got)
	}
	if got := envOrDefault("UC_TEST_MISSING", "fallback"); got != "fallback" {
		t.Fatalf("expected fallback value, got %q", got)
	}
}

func TestForwardedHeaderMatcher(t *testing.T) {
	for _, header := range []string{
		"traceparent",
		"tracestate",
		"x-correlation-id",
		"x-timeout-ms",
		"x-uc-cancelled",
	} {
		if _, ok := forwardedHeaderMatcher(header); !ok {
			t.Fatalf("expected %q to be forwarded", header)
		}
	}
}

func TestDocumentationEndpoints(t *testing.T) {
	server := httptest.NewServer(newHTTPMux(http.NotFoundHandler()))
	defer server.Close()

	response, err := http.Get(server.URL + "/openapi.json")
	if err != nil {
		t.Fatal(err)
	}
	defer response.Body.Close()
	body, err := io.ReadAll(response.Body)
	if err != nil {
		t.Fatal(err)
	}
	if response.StatusCode != http.StatusOK {
		t.Fatalf("expected OpenAPI 200, got %d", response.StatusCode)
	}
	if contentType := response.Header.Get("Content-Type"); contentType != "application/json" {
		t.Fatalf("expected JSON content type, got %q", contentType)
	}
	if !strings.Contains(string(body), `"/v1/runtime:ping"`) {
		t.Fatal("generated OpenAPI document must expose Ping")
	}

	response, err = http.Get(server.URL + "/swagger/")
	if err != nil {
		t.Fatal(err)
	}
	defer response.Body.Close()
	body, err = io.ReadAll(response.Body)
	if err != nil {
		t.Fatal(err)
	}
	if response.StatusCode != http.StatusOK {
		t.Fatalf("expected Swagger UI 200, got %d", response.StatusCode)
	}
	if !strings.Contains(string(body), `url: "/openapi.json"`) {
		t.Fatal("Swagger UI must load the gateway OpenAPI endpoint")
	}
}
