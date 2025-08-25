package genaip

import (
	"fmt"
	"strconv"
	"strings"

	"github.com/AnteWall/protoc-gen-rust-aip/pkg/resourcename"
	"google.golang.org/genproto/googleapis/api/annotations"
	"google.golang.org/protobuf/compiler/protogen"
	"google.golang.org/protobuf/reflect/protoregistry"
)

type resourceNameCodeGenerator struct {
	resource *annotations.ResourceDescriptor
	file     *protogen.File
	files    *protoregistry.Files
}

func (r resourceNameCodeGenerator) GenerateCode(g *protogen.GeneratedFile) error {
	if len(r.resource.GetPattern()) == 0 {
		return nil
	}

	hasMultiPattern := len(r.resource.GetPattern()) > 1
	hasFutureMultiPattern := r.resource.GetHistory() == annotations.ResourceDescriptor_FUTURE_MULTI_PATTERN

	// Generate multi-pattern trait if we have multiple patterns now or in the future.
	if hasMultiPattern || hasFutureMultiPattern {
		if err := r.generateMultiPatternTrait(g); err != nil {
			return err
		}
		if err := r.generateMultiPatternParseFunction(g); err != nil {
			return err
		}
	}

	// Generate the single-pattern struct only if this is truly a single-pattern resource
	firstPattern := r.resource.GetPattern()[0]
	shouldGenerateSinglePatternStruct := !hasFutureMultiPattern && !hasMultiPattern
	firstSinglePatternStructName := r.SinglePatternStructName()
	if shouldGenerateSinglePatternStruct {
		if err := r.generatePatternStruct(
			g, firstPattern, firstSinglePatternStructName,
		); err != nil {
			return err
		}
	}

	// Generate all pattern structs for multi-pattern resources
	if hasMultiPattern || hasFutureMultiPattern {
		for _, pattern := range r.resource.GetPattern() {
			if err := r.generatePatternStruct(g, pattern, r.MultiPatternStructName(pattern)); err != nil {
				return err
			}
		}
	}
	return nil
}

func (r resourceNameCodeGenerator) generatePatternStruct(
	g *protogen.GeneratedFile,
	pattern string,
	typeName string,
) error {
	g.P("/// Resource name for ", r.resource.GetType())
	g.P("#[derive(Debug, Clone, PartialEq, Eq, Hash)]")
	g.P("pub struct ", typeName, " {")

	var segments []resourcename.Segment
	if err := resourcename.ParsePattern(pattern, &segments); err != nil {
		return fmt.Errorf("failed to parse pattern %q: %w", pattern, err)
	}

	for _, segment := range segments {
		if segment.IsVariable() {
			fieldName := toRustFieldName(segment.Literal())
			g.P("    pub ", fieldName, ": String,")
		}
	}
	g.P("}")
	g.P()

	if err := r.generateImplBlock(g, pattern, typeName, segments); err != nil {
		return err
	}

	if err := r.generateDisplayTrait(g, pattern, typeName, segments); err != nil {
		return err
	}

	if err := r.generateFromStrTrait(g, pattern, typeName, segments); err != nil {
		return err
	}

	return nil
}

func (r resourceNameCodeGenerator) generateImplBlock(
	g *protogen.GeneratedFile,
	pattern string,
	typeName string,
	segments []resourcename.Segment,
) error {
	g.P("impl ", typeName, " {")

	// Generate constructor
	r.generateConstructor(g, typeName, segments)

	// Generate validation method
	r.generateValidateMethod(g, typeName, segments)

	// Generate type method
	g.P("    /// Returns the resource type.")
	g.P("    pub fn resource_type(&self) -> &'static str {")
	g.P("        ", strconv.Quote(r.resource.GetType()))
	g.P("    }")
	g.P()

	// Generate contains_wildcard method
	r.generateContainsWildcardMethod(g, typeName, segments)

	g.P("}")
	g.P()
	return nil
}

func (r resourceNameCodeGenerator) generateConstructor(
	g *protogen.GeneratedFile,
	typeName string,
	segments []resourcename.Segment,
) {
	g.P("    /// Creates a new ", typeName, ".")
	g.P("    pub fn new(")

	var params []string
	for _, segment := range segments {
		if segment.IsVariable() {
			fieldName := toRustFieldName(segment.Literal())
			params = append(params, fieldName+": impl Into<String>")
		}
	}

	for i, param := range params {
		if i == len(params)-1 {
			g.P("        ", param)
		} else {
			g.P("        ", param, ",")
		}
	}

	g.P("    ) -> Self {")
	g.P("        Self {")
	for _, segment := range segments {
		if segment.IsVariable() {
			fieldName := toRustFieldName(segment.Literal())
			g.P("            ", fieldName, ": ", fieldName, ".into(),")
		}
	}
	g.P("        }")
	g.P("    }")
	g.P()
}

func (r resourceNameCodeGenerator) generateValidateMethod(
	g *protogen.GeneratedFile,
	typeName string,
	segments []resourcename.Segment,
) {
	g.P("    /// Validates the resource name.")
	g.P("    pub fn validate(&self) -> Result<(), String> {")
	for _, segment := range segments {
		if segment.IsVariable() {
			fieldName := toRustFieldName(segment.Literal())
			g.P("        if self.", fieldName, ".is_empty() {")
			g.P("            return Err(\"", segment.Literal(), ": empty\".to_string());")
			g.P("        }")
			g.P("        if self.", fieldName, ".contains('/') {")
			g.P("            return Err(\"", segment.Literal(), ": contains illegal character '/'\".to_string());")
			g.P("        }")
		}
	}
	g.P("        Ok(())")
	g.P("    }")
	g.P()
}

func (r resourceNameCodeGenerator) generateContainsWildcardMethod(
	g *protogen.GeneratedFile,
	typeName string,
	segments []resourcename.Segment,
) {
	g.P("    /// Returns true if any field contains a wildcard.")
	g.P("    pub fn contains_wildcard(&self) -> bool {")

	var conditions []string
	for _, segment := range segments {
		if segment.IsVariable() {
			fieldName := toRustFieldName(segment.Literal())
			conditions = append(conditions, "self."+fieldName+" == \"-\"")
		}
	}

	if len(conditions) == 0 {
		g.P("        false")
	} else {
		g.P("        ", strings.Join(conditions, " || "))
	}
	g.P("    }")
	g.P()
}

func (r resourceNameCodeGenerator) generateDisplayTrait(
	g *protogen.GeneratedFile,
	pattern string,
	typeName string,
	segments []resourcename.Segment,
) error {
	g.P("impl fmt::Display for ", typeName, " {")
	g.P("    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {")

	// Build the format string with inlined variables
	var formatStr strings.Builder
	first := true
	for _, segment := range segments {
		if !first {
			formatStr.WriteString("/")
		}
		if segment.IsVariable() {
			fieldName := toRustFieldName(segment.Literal())
			formatStr.WriteString("{" + fieldName + "}")
		} else {
			formatStr.WriteString(segment.Literal())
		}
		first = false
	}

	// Use inlined format args for clippy compliance
	g.P("        write!(f, \"", formatStr.String(), "\"")

	// Add field references for inlined format
	for _, segment := range segments {
		if segment.IsVariable() {
			fieldName := toRustFieldName(segment.Literal())
			g.P("            , ", fieldName, " = self.", fieldName)
		}
	}

	g.P("        )")
	g.P("    }")
	g.P("}")
	g.P()
	return nil
}

func (r resourceNameCodeGenerator) generateFromStrTrait(
	g *protogen.GeneratedFile,
	pattern string,
	typeName string,
	segments []resourcename.Segment,
) error {
	g.P("impl FromStr for ", typeName, " {")
	g.P("    type Err = String;")
	g.P()
	g.P("    fn from_str(s: &str) -> Result<Self, Self::Err> {")

	// Generate pattern matching logic
	g.P("        let parts: Vec<&str> = s.split('/').collect();")

	// Count expected parts
	expectedParts := len(segments)

	g.P("        if parts.len() != ", expectedParts, " {")
	g.P("            return Err(format!(\"expected {expected_parts} parts, got {}\", parts.len(), expected_parts = ", expectedParts, "));")
	g.P("        }")

	// Extract variables
	for i, segment := range segments {
		if segment.IsVariable() {
			fieldName := toRustFieldName(segment.Literal())
			g.P("        let ", fieldName, " = parts[", i, "].to_string();")
		} else {
			g.P("        if parts[", i, "] != ", strconv.Quote(segment.Literal()), " {")
			g.P("            return Err(format!(\"expected '", segment.Literal(), "' at position ", i, ", got '{part}'\", part = parts[", i, "]));")
			g.P("        }")
		}
	}

	g.P("        let result = Self {")
	for _, segment := range segments {
		if segment.IsVariable() {
			fieldName := toRustFieldName(segment.Literal())
			g.P("            ", fieldName, ",")
		}
	}
	g.P("        };")
	g.P("        result.validate()?;")
	g.P("        Ok(result)")
	g.P("    }")
	g.P("}")
	g.P()
	return nil
}

func (r resourceNameCodeGenerator) generateMultiPatternTrait(g *protogen.GeneratedFile) error {
	// Generate an enum instead of a trait for object-safety
	enumName := r.MultiPatternEnumName()
	g.P("/// Multi-pattern resource name for ", r.resource.GetType())
	g.P("#[derive(Debug, Clone, PartialEq, Eq, Hash)]")
	g.P("pub enum ", enumName, " {")

	for _, pattern := range r.resource.GetPattern() {
		structName := r.MultiPatternStructName(pattern)
		variantName := r.getPatternVariantName(pattern)
		g.P("    ", variantName, "(", structName, "),")
	}

	g.P("}")
	g.P()

	// Generate implementations for the enum
	r.generateMultiPatternEnumImpls(g, enumName)

	return nil
}

func (r resourceNameCodeGenerator) generateMultiPatternEnumImpls(g *protogen.GeneratedFile, enumName string) {
	g.P("impl ", enumName, " {")
	g.P("    /// Returns the resource type.")
	g.P("    pub fn resource_type(&self) -> &'static str {")
	g.P("        ", strconv.Quote(r.resource.GetType()))
	g.P("    }")
	g.P()
	g.P("    /// Returns true if any field contains a wildcard.")
	g.P("    pub fn contains_wildcard(&self) -> bool {")
	g.P("        match self {")
	for _, pattern := range r.resource.GetPattern() {
		variantName := r.getPatternVariantName(pattern)
		g.P("            ", enumName, "::", variantName, "(inner) => inner.contains_wildcard(),")
	}
	g.P("        }")
	g.P("    }")
	g.P()
	g.P("    /// Validates the resource name.")
	g.P("    pub fn validate(&self) -> Result<(), String> {")
	g.P("        match self {")
	for _, pattern := range r.resource.GetPattern() {
		variantName := r.getPatternVariantName(pattern)
		g.P("            ", enumName, "::", variantName, "(inner) => inner.validate(),")
	}
	g.P("        }")
	g.P("    }")
	g.P("}")
	g.P()

	// Generate Display trait
	g.P("impl fmt::Display for ", enumName, " {")
	g.P("    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {")
	g.P("        match self {")
	for _, pattern := range r.resource.GetPattern() {
		variantName := r.getPatternVariantName(pattern)
		g.P("            ", enumName, "::", variantName, "(inner) => write!(f, \"{inner}\"),")
	}
	g.P("        }")
	g.P("    }")
	g.P("}")
	g.P()

	// Generate FromStr trait
	g.P("impl FromStr for ", enumName, " {")
	g.P("    type Err = String;")
	g.P()
	g.P("    fn from_str(s: &str) -> Result<Self, Self::Err> {")
	for _, pattern := range r.resource.GetPattern() {
		structName := r.MultiPatternStructName(pattern)
		variantName := r.getPatternVariantName(pattern)
		g.P("        if let Ok(parsed) = ", structName, "::from_str(s) {")
		g.P("            return Ok(", enumName, "::", variantName, "(parsed));")
		g.P("        }")
	}
	g.P("        Err(\"no matching pattern\".to_string())")
	g.P("    }")
	g.P("}")
	g.P()
}

func (r resourceNameCodeGenerator) generateMultiPatternParseFunction(g *protogen.GeneratedFile) error {
	enumName := r.MultiPatternEnumName()
	g.P("/// Parses a resource name string and returns the appropriate type.")
	g.P("pub fn parse_", toSnakeCase(r.getResourceKind()), "_resource_name(name: &str) -> Result<", enumName, ", String> {")
	g.P("    ", enumName, "::from_str(name)")
	g.P("}")
	g.P()
	return nil
}

func (r *resourceNameCodeGenerator) SinglePatternStructName() string {
	return toPascalCase(r.getResourceKind()) + "ResourceName"
}

func (r *resourceNameCodeGenerator) MultiPatternStructName(pattern string) string {
	if r.resource.GetHistory() == annotations.ResourceDescriptor_FUTURE_MULTI_PATTERN || len(r.resource.GetPattern()) > 1 {
		var result strings.Builder
		var segments []resourcename.Segment
		if err := resourcename.ParsePattern(pattern, &segments); err == nil {
			for _, segment := range segments {
				if !segment.IsVariable() && segment.Literal() != r.resource.GetPlural() {
					result.WriteString(toPascalCase(segment.Literal()))
				}
			}
		}

		// If no prefix found, use the collection name
		if result.Len() == 0 {
			if err := resourcename.ParsePattern(pattern, &segments); err == nil && len(segments) > 0 {
				if !segments[0].IsVariable() {
					result.WriteString(toPascalCase(segments[0].Literal()))
				} else {
					result.WriteString("Simple")
				}
			} else {
				result.WriteString("Default")
			}
		}

		result.WriteString(r.SinglePatternStructName())
		return result.String()
	}
	if len(r.resource.GetPattern()) > 0 && r.resource.GetPattern()[0] == pattern {
		return r.SinglePatternStructName()
	}
	return r.MultiPatternStructName(pattern)
}

func (r *resourceNameCodeGenerator) MultiPatternTraitName() string {
	return toPascalCase(r.getResourceKind()) + "ResourceName"
}

func (r *resourceNameCodeGenerator) MultiPatternEnumName() string {
	return toPascalCase(r.getResourceKind()) + "ResourceName"
}

func (r *resourceNameCodeGenerator) getPatternVariantName(pattern string) string {
	var result strings.Builder
	var segments []resourcename.Segment
	if err := resourcename.ParsePattern(pattern, &segments); err == nil {
		for _, segment := range segments {
			if !segment.IsVariable() && segment.Literal() != r.resource.GetPlural() {
				result.WriteString(toPascalCase(segment.Literal()))
			}
		}
	}
	if result.Len() == 0 {
		// If no prefix was found, generate a name based on the pattern structure
		// For patterns like "authors/{author}", use the collection as the variant name
		if err := resourcename.ParsePattern(pattern, &segments); err == nil && len(segments) > 0 {
			if !segments[0].IsVariable() {
				result.WriteString(toPascalCase(segments[0].Literal()))
			} else {
				result.WriteString("Simple")
			}
		} else {
			result.WriteString("Default")
		}
	}
	return result.String()
}

func (r *resourceNameCodeGenerator) getResourceKind() string {
	parts := strings.Split(r.resource.GetType(), "/")
	if len(parts) >= 2 {
		return parts[1]
	}
	return "Resource"
}

// Helper functions for Rust naming conventions
func toRustFieldName(s string) string {
	return toSnakeCase(s)
}

func toSnakeCase(s string) string {
	var result strings.Builder
	for i, c := range s {
		if i > 0 && 'A' <= c && c <= 'Z' {
			result.WriteByte('_')
		}
		if 'A' <= c && c <= 'Z' {
			result.WriteByte(byte(c - 'A' + 'a'))
		} else {
			result.WriteRune(c)
		}
	}
	return result.String()
}

func toPascalCase(s string) string {
	var result strings.Builder
	capitalize := true
	for _, c := range s {
		if c == '_' || c == '-' {
			capitalize = true
		} else if capitalize {
			if 'a' <= c && c <= 'z' {
				result.WriteRune(c - 'a' + 'A')
			} else {
				result.WriteRune(c)
			}
			capitalize = false
		} else {
			result.WriteRune(c)
		}
	}
	return result.String()
}
