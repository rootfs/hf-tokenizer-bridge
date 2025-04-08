package tokenizer

// #cgo LDFLAGS: -L${SRCDIR}/lib -lhf_tokenizer_bridge
// #include <stdlib.h>
// #include <stdio.h>
// #include "hf_tokenizer_bridge.h"
import "C"
import (
	"encoding/json"
	"errors"
	"fmt"
	"os"
	"strings"
	"unsafe"
)

// TokenizerResult represents the result of tokenization
type TokenizerResult struct {
	Tokens    []string `json:"tokens"`
	Ids       []int    `json:"ids"`
	Error     string   `json:"error,omitempty"`
	DebugLogs []string `json:"debug_logs,omitempty"`
}

// Tokenize tokenizes text using the specified Hugging Face model
func Tokenize(text string, modelName string) (*TokenizerResult, error) {
	cText := C.CString(text)
	defer C.free(unsafe.Pointer(cText))

	cModelName := C.CString(modelName)
	defer C.free(unsafe.Pointer(cModelName))

	// Redirect stderr to capture Rust's eprintln output
	// Remove or comment this out to see Rust's eprintln in the terminal
	// This is mainly to show we're capturing the messages in our debug logs
	originalStderr, _ := os.Open("/dev/stderr")
	defer originalStderr.Close()

	cResult := C.tokenize_text(cText, cModelName)

	// Restore stderr

	if cResult == nil {
		return nil, errors.New("tokenization failed - could not load tokenizer (nil result)")
	}
	defer C.free_string(cResult)

	resultJSON := C.GoString(cResult)
	fmt.Println("Raw JSON result:", resultJSON)

	var result TokenizerResult
	err := json.Unmarshal([]byte(resultJSON), &result)
	if err != nil {
		return nil, fmt.Errorf("error parsing result: %v, raw JSON: %s", err, resultJSON)
	}

	// Print debug logs if they exist
	if len(result.DebugLogs) > 0 {
		fmt.Println("Debug logs from Rust:")
		for _, log := range result.DebugLogs {
			fmt.Println("  →", log)
		}
		fmt.Println(strings.Repeat("-", 40))
	}

	if result.Error != "" {
		return nil, fmt.Errorf("tokenization error: %s", result.Error)
	}

	return &result, nil
}

// TokenizeWithToken tokenizes text using the specified Hugging Face model and authentication token
func TokenizeWithToken(text string, modelName string, token string) (*TokenizerResult, error) {
	cText := C.CString(text)
	defer C.free(unsafe.Pointer(cText))

	cModelName := C.CString(modelName)
	defer C.free(unsafe.Pointer(cModelName))

	cToken := C.CString(token)
	defer C.free(unsafe.Pointer(cToken))

	cResult := C.tokenize_text_with_token(cText, cModelName, cToken)

	if cResult == nil {
		return nil, errors.New("tokenization failed - could not load tokenizer (nil result)")
	}
	defer C.free_string(cResult)

	resultJSON := C.GoString(cResult)
	fmt.Println("Raw JSON result:", resultJSON)

	var result TokenizerResult
	err := json.Unmarshal([]byte(resultJSON), &result)
	if err != nil {
		return nil, fmt.Errorf("error parsing result: %v, raw JSON: %s", err, resultJSON)
	}

	// Print debug logs if they exist
	if len(result.DebugLogs) > 0 {
		fmt.Println("Debug logs from Rust:")
		for _, log := range result.DebugLogs {
			fmt.Println("  →", log)
		}
		fmt.Println(strings.Repeat("-", 40))
	}

	if result.Error != "" {
		return nil, fmt.Errorf("tokenization error: %s", result.Error)
	}

	return &result, nil
}
