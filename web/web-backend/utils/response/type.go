package response

// Type 也就是ResponseType 标识返回类型
type Type int

const TypeJSON Type = 1
const TypeFile Type = 2
const TypeFailed Type = 3
const TypeRedirect Type = 4
const TypeImage Type = 5

func (t Type) String() string {
	switch t {
	case TypeJSON:
		return "JSON"
	case TypeFile:
		return "File"
	case TypeFailed:
		return "Failed"
	case TypeRedirect:
		return "Redirect"
	case TypeImage:
		return "Images"
	default:
		return "<unknown>"
	}
}
