package main

import (
	"testing"
)

// TestDefunct always passes.
func TestDefunct(t *testing.T) {
	if false {
		t.Fatalf(`An unreachable branch was executed.`)
	}
}
