package main

import (
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
