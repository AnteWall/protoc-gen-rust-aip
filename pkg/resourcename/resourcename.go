// Package resourcename implements simple functions to manipulate UTF-8 encoded AIP resource names.
package resourcename

import (
	"fmt"
	"strings"
)

// Segment represents a segment of a resource name pattern.
type Segment struct {
	literal    string
	isVariable bool
}

// NewSegment creates a new segment.
func NewSegment(literal string, isVariable bool) Segment {
	return Segment{
		literal:    literal,
		isVariable: isVariable,
	}
}

// IsVariable returns true if this segment is a variable (e.g., {project}).
func (s Segment) IsVariable() bool {
	return s.isVariable
}

// Literal returns the literal value of the segment.
func (s Segment) Literal() string {
	return s.literal
}

// ParsePattern parses a resource name pattern into segments.
func ParsePattern(pattern string, segments *[]Segment) error {
	if segments == nil {
		return fmt.Errorf("segments slice cannot be nil")
	}

	*segments = nil
	parts := strings.Split(pattern, "/")

	for _, part := range parts {
		if strings.HasPrefix(part, "{") && strings.HasSuffix(part, "}") {
			// Variable segment
			literal := part[1 : len(part)-1]
			*segments = append(*segments, NewSegment(literal, true))
		} else {
			// Literal segment
			*segments = append(*segments, NewSegment(part, false))
		}
	}

	return nil
}

// Wildcard is the resource name wildcard character "-".
const Wildcard = "-"

// Sprint formats resource name variables according to a pattern and returns the resulting string.
func Sprint(pattern string, variables ...string) string {
	var segments []Segment
	if err := ParsePattern(pattern, &segments); err != nil {
		return ""
	}

	var result strings.Builder
	varIndex := 0

	for i, segment := range segments {
		if i > 0 {
			result.WriteByte('/')
		}

		if segment.IsVariable() {
			if varIndex < len(variables) {
				result.WriteString(variables[varIndex])
				varIndex++
			}
		} else {
			result.WriteString(segment.Literal())
		}
	}

	return result.String()
}

// Sscan scans a resource name, storing successive segments into successive variables
// as determined by the provided pattern.
func Sscan(name, pattern string, variables ...*string) error {
	var segments []Segment
	if err := ParsePattern(pattern, &segments); err != nil {
		return err
	}

	parts := strings.Split(name, "/")
	if len(parts) != len(segments) {
		return fmt.Errorf("expected %d parts, got %d", len(segments), len(parts))
	}

	varIndex := 0
	for i, segment := range segments {
		if segment.IsVariable() {
			if varIndex < len(variables) && variables[varIndex] != nil {
				*variables[varIndex] = parts[i]
				varIndex++
			}
		} else {
			if parts[i] != segment.Literal() {
				return fmt.Errorf("expected %q at position %d, got %q", segment.Literal(), i, parts[i])
			}
		}
	}

	return nil
}

// Validate validates a resource name format.
func Validate(name string) error {
	if name == "" {
		return fmt.Errorf("resource name cannot be empty")
	}

	if strings.Contains(name, "//") {
		return fmt.Errorf("resource name cannot contain empty segments")
	}

	if strings.HasPrefix(name, "/") || strings.HasSuffix(name, "/") {
		return fmt.Errorf("resource name cannot start or end with '/'")
	}

	return nil
}

// Match tests whether a resource name matches a pattern.
func Match(pattern, name string) bool {
	var segments []Segment
	if err := ParsePattern(pattern, &segments); err != nil {
		return false
	}

	parts := strings.Split(name, "/")
	if len(parts) != len(segments) {
		return false
	}

	for i, segment := range segments {
		if !segment.IsVariable() && segment.Literal() != parts[i] {
			return false
		}
	}

	return true
}
