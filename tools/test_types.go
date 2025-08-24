package tools
package main

import (
    "fmt"
    "reflect"
    "google.golang.org/genproto/googleapis/api/annotations"
)

func main() {
    rd := &annotations.ResourceDescriptor{}
    fmt.Printf("History type: %s\n", reflect.TypeOf(rd.GetHistory()))
    fmt.Printf("History value: %v\n", rd.GetHistory())
    
    // Try to figure out the actual field type
    rdType := reflect.TypeOf(rd).Elem()
    historyField, found := rdType.FieldByName("History")
    if found {
        fmt.Printf("History field type: %s\n", historyField.Type)
    }
    
    // Show all the history enum values
    fmt.Printf("History enum values:\n")
    fmt.Printf("HISTORY_UNSPECIFIED: %v\n", annotations.ResourceDescriptor_HISTORY_UNSPECIFIED)
    fmt.Printf("ORIGINALLY_SINGLE_PATTERN: %v\n", annotations.ResourceDescriptor_ORIGINALLY_SINGLE_PATTERN) 
    fmt.Printf("FUTURE_MULTI_PATTERN: %v\n", annotations.ResourceDescriptor_FUTURE_MULTI_PATTERN)
}
