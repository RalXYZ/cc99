var id;

export default function Ast2Vis(ast) {
  id = 0;
  let root = {
    id: id.toString(),
    label: "CC99",
    attrs: {
      remark: "AST ROOT NODE",
    },
    children: node2tree(ast.GlobalDeclaration),
  };
  return root;
}

function node2tree(astNode) {
  let nodeList = [];
  for (const node of astNode) {
    if (!node) {
      continue;
    }
    id += 1;
    let treeNode = {
      id: id.toString(),
      label: "",
      attrs: {},
      children: [],
    };
    if (node.hasOwnProperty("Declaration")) {
      treeNode.label = "Declaration";
      const [type, name, expression] = node.Declaration;
      treeNode.attrs = { name: name, ...parseType(type) };

      treeNode.children = node2tree([expression]);
    } else if (node.hasOwnProperty("FunctionDefinition")) {
      treeNode.label = "Function";
      const [specifier, storage, return_type, name, p, isVariadic, body] =
        node.FunctionDefinition;
      let paramsList = {};
      let anonyvar = 1;
      p.forEach((param) => {
        if (param[1]) {
          paramsList[param[1]] = parseBasicType(param[0]);
        } else {
          paramsList["anony_var" + anonyvar] = parseBasicType(param[0]);
          anonyvar += 1;
        }
      });
      treeNode.attrs = {
        name: name,
        storage: storage,
        return_type: parseBasicType(return_type),
        params: paramsList,
        is_variadic: isVariadic,
      };
      if (specifier.length > 0) {
        treeNode.attrs.specifier = specifier.join(" ");
      }
      treeNode.children = node2tree([body]);
    } else if (node.hasOwnProperty("Labeled")) {
      treeNode.label = "Labeled";
      const [name, statement] = node.Labeledl;
      treeNode.attrs = {
        name: name,
      };
      treeNode.children = node2tree([statement]);
    } else if (node.hasOwnProperty("Case")) {
      treeNode.label = "Case";
      const [expression, statements] = node.Case;
      treeNode.children = node2tree([expression, statements]);
    } else if (node.hasOwnProperty("Compound")) {
      treeNode.label = "Compound";
      treeNode.children = node2tree(node.Compound);
    } else if (node.hasOwnProperty("Expression")) {
      treeNode.label = "Expression";
      treeNode.children = node2tree([node.Expression]);
    } else if (node.hasOwnProperty("If")) {
      treeNode.label = "If";
      const [expression, statement, elseStatement] = node.If;
      treeNode.children = node2tree([expression, statement, elseStatement]);
    } else if (node.hasOwnProperty("Switch")) {
      treeNode.label = "Switch";
      const [expression, statement] = node.Switch;
      treeNode.children = node2tree([expression, statement]);
    } else if (node.hasOwnProperty("While")) {
      treeNode.label = "While";
      const [expression, statement] = node.While;
      treeNode.children = node2tree([expression, statement]);
    } else if (node.hasOwnProperty("DoWhile")) {
      treeNode.label = "DoWhile";
      const [statement, expression] = node.DoWhile;
      treeNode.children = node2tree([statement, expression]);
    } else if (node.hasOwnProperty("For")) {
      treeNode.label = "For";
      const [init, condition, iter, statement] = node.For;
      treeNode.children = node2tree([init, condition, iter, statement]);
    } else if (node === "Break") {
      treeNode.label = "Break";
    } else if (node === "Continue") {
      treeNode.label = "Continue";
    } else if (node.hasOwnProperty("Return")) {
      treeNode.label = "Return";
      const expression = node.Return;
      treeNode.children = node2tree([expression]);
    } else if (node.hasOwnProperty("Goto")) {
      treeNode.label = "Goto";
      const label = node.Goto;
      treeNode.attrs.label = label;
    } else if (node.hasOwnProperty("Assignment")) {
      treeNode.label = "Assignment";
      const [op, lhs, rhs] = node.Assignment;
      treeNode.attrs.operation = op;
      treeNode.children = node2tree([lhs, rhs]);
    } else if (node.hasOwnProperty("Unary")) {
      treeNode.label = "Unary";
      treeNode.Unary = "Unary";
      const [op, expression] = node.Unary;
      treeNode.attrs.operation = op;
      treeNode.children = node2tree([expression]);
    } else if (node.hasOwnProperty("Binary")) {
      treeNode.label = "Binary";
      const [op, lhs, rhs] = node.Binary;
      treeNode.attrs.operation = op;
      treeNode.children = node2tree([lhs, rhs]);
    } else if (node.hasOwnProperty("FunctionCall")) {
      treeNode.label = "FunctionCall";
      const [expression, args] = node.FunctionCall;
      treeNode.children = node2tree([expression, ...args]);
    } else if (node.hasOwnProperty("TypeCast")) {
      treeNode.label = "TypeCast";
      const [basicType, expression] = node.TypeCast;
      treeNode.attrs.basic_type = parseBasicType(basicType);
      treeNode.children = node2tree([expression]);
    } else if (node.hasOwnProperty("Conditional")) {
      treeNode.label = "Conditional";
      const [condition, then, else_] = node.Conditional;
      treeNode.children = node2tree([condition, then, else_]);
    } else if (node.hasOwnProperty("SizeofType")) {
      treeNode.label = "SizeofType";
      const basicType = node.SizeofType;
      treeNode.attrs.basic_type = parseBasicType(basicType);
    } else if (node.hasOwnProperty("MemberOfObject")) {
      treeNode.label = "MemberOfObject";
      const [object, member] = node.MemberOfObject;
      treeNode.attrs.MemberName = member;
      treeNode.children = node2tree([object]);
    } else if (node.hasOwnProperty("MemberOfPointer")) {
      treeNode.label = "MemberOfPointer";
      const [pointer, member] = node.MemberOfPointer;
      treeNode.attrs.MemberName = member;
      treeNode.children = node2tree([pointer]);
    } else if (node.hasOwnProperty("ArraySubscript")) {
      treeNode.label = "ArraySubscript";
      const [array, subscript] = node.ArraySubscript;
      treeNode.children = node2tree([array, ...subscript]);
    } else if (node.hasOwnProperty("Identifier")) {
      treeNode.label = "Identifier";
      const name = node.Identifier;
      treeNode.attrs.name = name;
    } else if (node.hasOwnProperty("IntegerConstant")) {
      treeNode.label = "IntegerConstant";
      const value = node.IntegerConstant;
      treeNode.attrs.value = value;
    } else if (node.hasOwnProperty("UnsignedIntegerConstant")) {
      treeNode.label = "UnsignedIntegerConstant";
      const value = node.UnsignedIntegerConstant;
      treeNode.attrs.value = value;
    } else if (node.hasOwnProperty("LongConstant")) {
      treeNode.label = "LongConstant";
      const value = node.LongConstant;
      treeNode.attrs.value = value;
    } else if (node.hasOwnProperty("LongLongConstant")) {
      treeNode.label = "LongLongConstant";
      const value = node.LongLongConstant;
      treeNode.attrs.value = value;
    } else if (node.hasOwnProperty("FloatConstant")) {
      treeNode.label = "FloatConstant";
      const value = node.FloatConstant;
      treeNode.attrs.value = value;
    } else if (node.hasOwnProperty("DoubleConstant")) {
      treeNode.label = "DoubleConstant";
      const value = node.DoubleConstant;
      treeNode.attrs.value = value;
    } else if (node.hasOwnProperty("UnsignedLongConstant")) {
      treeNode.label = "UnsignedLongConstant";
      const value = node.UnsignedLongConstant;
      treeNode.attrs.value = value;
    } else if (node.hasOwnProperty("UnsignedLongLongConstant")) {
      treeNode.label = "UnsignedLongLongConstant";
      const value = node.UnsignedLongLongConstant;
      treeNode.attrs.value = value;
    } else if (node.hasOwnProperty("CharacterConstant")) {
      treeNode.label = "CharacterConstant";
      const value = node.CharacterConstant;
      treeNode.attrs.value = value;
    } else if (node.hasOwnProperty("StringLiteral")) {
      treeNode.label = "StringLiteral";
      const value = node.StringLiteral;
      treeNode.attrs.value = value;
    } else if (node === "Empty") {
      treeNode.label = "Empty";
    } else if (node.hasOwnProperty("ForDeclaration")) {
      treeNode.label = "ForDeclaration";
      treeNode.children = node2tree(node.ForDeclaration);
    } else if (node.hasOwnProperty("LocalDeclaration")) {
      treeNode.label = "LocalDeclaration";
      const Declaration = node.LocalDeclaration;
      treeNode.children = node2tree([Declaration]);
    } else if (node.hasOwnProperty("Statement")) {
      treeNode.label = "Statement";
      const Statement = node.Statement;
      treeNode.children = node2tree([Statement]);
    }

    nodeList.push(treeNode);
  }
  return nodeList;
}

function parseType(node) {
  let attrs = {
    storageClass: node.storage_class_specifier,
  };
  if (node.function_specifier.length > 0) {
    attrs.specifier = node.function_specifier.join(" ");
  }
  attrs.basic_type = parseBasicType(node.basic_type);
  return attrs;
}

function parseBasicType(node) {
  let attrs = {};
  if (node.qualifier.length > 0) {
    attrs.qualifier = node.qualifier.join(" ");
  }
  let basic_type = node.base_type;
  if (basic_type.hasOwnProperty("Void")) {
    attrs.type = "void";
  } else if (basic_type.hasOwnProperty("SignedInteger")) {
    attrs.type = "signed_" + basic_type.SignedInteger;
  } else if (basic_type.hasOwnProperty("UnsignedInteger")) {
    attrs.type = "unsigned_" + basic_type.UnsignedInteger;
  } else if (basic_type.hasOwnProperty("Bool")) {
    attrs.type = "bool";
  } else if (basic_type.hasOwnProperty("Float")) {
    attrs.type = "float";
  } else if (basic_type.hasOwnProperty("Double")) {
    attrs.type = "double";
  } else if (basic_type.hasOwnProperty("Point")) {
    attrs.type = "point";
    const bt = basic_type.Point;
    attrs.basic_type = parseBasicType(bt);
  } else if (basic_type.hasOwnProperty("Array")) {
    attrs.type = "array";
    const [bt, dimension_expr] = basic_type.Array;
    attrs.dimension = dimension_expr.length;
    attrs.basic_type = parseBasicType(bt);
  } else if (basic_type.hasOwnProperty("Function")) {
    attrs.type = "function";
    const [return_type, param_type, isVariadic] = basic_type.Function;
    attrs.return_type = parseBasicType(return_type);
    for (let i = 0; i < param_type.length; i++) {
      attrs["param" + i] = parseBasicType(param_type[i]);
    }
    attrs.variadic = isVariadic;
  } else if (basic_type.hasOwnProperty("Struct")) {
    attrs.type = "struct";
    const [struct_name, members] = basic_type.Struct;
    attrs.struct_name = struct_name;
    for (let member of members) {
      attrs[member.member_name] = parseBasicType(member.member_type);
    }
  } else if (basic_type.hasOwnProperty("Union")) {
    attrs.type = "union";
    const [union_name, members] = basic_type.Union;
    attrs.union_name = union_name;
    for (let member of members) {
      attrs[member.member_name] = parseBasicType(member.member_type);
    }
  } else if (basic_type.hasOwnProperty("Identifier")) {
    attrs.type = "identifier";
    attrs.name = basic_type.Identifier;
  }
  return attrs;
}
