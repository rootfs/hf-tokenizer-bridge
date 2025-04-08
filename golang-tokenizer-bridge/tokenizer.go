package tokenizer

// #cgo LDFLAGS: -L${SRCDIR}/lib -lhf_tokenizer_bridge
// #include <stdlib.h>
// #include "hf_tokenizer_bridge.h"
import "C"
import (
	"encoding/json"
	"unsafe"
)

// TokenizerResult represents the result of tokenization
type TokenizerResult struct {
	Tokens []string `json:"tokens"`
	Ids    []int    `json:"ids"`
}

// Tokenize tokenizes text using the specified Hugging Face model
func Tokenize(text string, modelName string) (*TokenizerResult, error) {
	cText := C.CString(text)
	defer C.free(unsafe.Pointer(cText))

	cModelName := C.CString(modelName)
	defer C.free(unsafe.Pointer(cModelName))

	cResult := C.tokenize_text(cText, cModelName)
	if cResult == nil {
		return nil, nil
	}
	defer C.free_string(cResult)

	resultJSON := C.GoString(cResult)

	var result TokenizerResult
	err := json.Unmarshal([]byte(resultJSON), &result)
	if err != nil {
		return nil, err
	}

	return &result, nil
}
