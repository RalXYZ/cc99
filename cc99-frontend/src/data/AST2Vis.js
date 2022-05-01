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
}

function node2tree(astNode) {
  let nodeList = [];
  for (const node of astNode) {
    id += 1;
    let treeNode = {
      id: id.toString(),
      label: "",
      attrs: {},
      children: [],
    };
    if (node.Declaration) {
      treeNode.label = "Declaration";
      const [type, name, expression] = node.Declaration;
      treeNode.attrs.name = name;
      treeNode.children = node2tree([type, expression]);
    } else if (node.FunctionDefinition) {
      treeNode.label = "Function";
      const [type, name, params, body] = node.FunctionDefinition;
      treeNode.attrs = {
        name: name,
        params: params,
      };
      treeNode.children = node2tree([type, body]);
    } else if (node.Labeled) {
      treeNode.label = "Labeled";
      const [name, statement] = node.Labeledl;
      treeNode.attrs = {
        name: name,
      };
      treeNode.children = node2tree([statement]);
    } else if (node.Case) {
      treeNode.label = "Case";
      const [expression, statements] = node.Case;
      treeNode.children = node2tree([expression, statements]);
    } else if (node.Compound) {
      treeNode.label = "Compound";
      treeNode.children = node2tree(node.Compound);
    } else if (node.Expression) {
      treeNode.label = "Expression";
      treeNode.children = node2tree([node.Expression]);
    } else if (node.If) {
      treeNode.label = "If";
      const [expression, statement, elseStatement] = node.If;
      treeNode.children = node2tree([expression, statement, elseStatement]);
    } else if (node.Switch) {
      treeNode.label = "Switch";
      const [expression, statement] = node.Switch;
      treeNode.children = node2tree([expression, statement]);
    } else if (node.While) {
      treeNode.label = "While";
      const [expression, statement] = node.While;
      treeNode.children = node2tree([expression, statement]);
    } else if (node.DoWhile) {
      treeNode.label = "DoWhile";
      const [statement, expression] = node.DoWhile;
      treeNode.children = node2tree([statement, expression]);
    } else if (node.For) {
      treeNode.label = "For";
      const [init, condition, iter, statement] = node.For;
      treeNode.children = node2tree([init, condition, iter, statement]);
    } else if (node.Break) {
      treeNode.label = "Break";
    } else if (node.Continue) {
      treeNode.label = "Continue";
    } else if (node.Return) {
      treeNode.label = "Return";
      const [expression] = node.Return;
      treeNode.children = node2tree([expression]);
    } else if (node.Goto) {
      treeNode.label = "Goto";
      const [label] = node.Goto;
      treeNode.attrs.label = label;
    } else if (node.Assignment) {
      treeNode.label = "Assignment";
      const [op, lhs, rhs] = node.Assignment;
      treeNode.attrs.operation = op;
      treeNode.children = node2tree([lhs, rhs]);
    } else if (node.Unary) {
      treeNode.Unary = "Unary";
      const [op, expression] = node.Unary;
      treeNode.attrs.operation = op;
      treeNode.children = node2tree([expression]);
    } else if (node.Binary) {
      treeNode.label = "Binary";
      const [op, lhs, rhs] = node.Binary;
      treeNode.attrs.operation = op;
      treeNode.children = node2tree([lhs, rhs]);
    } else if (node.FunctionCall) {
      treeNode.label = "FunctionCall";
      const [expression, args] = node.FunctionCall;
      treeNode.children = node2tree([expression, args]);
    } else if (node.TypeCast) {
      treeNode.label = "TypeCast";
      const [basicType, expression] = node.TypeCast;
      treeNode.children = node2tree([basicType, expression]);
    } else if (node.Conditional) {
      treeNode.label = "Conditional";
      const [condition, then, else_] = node.Conditional;
      treeNode.children = node2tree([condition, then, else_]);
    } else if (node.SizeofType) {
      treeNode.label = "SizeofType";
      const [basicType] = node.SizeofType;
      treeNode.children = node2tree([basicType]);
    } else if (node.MemberOfObject) {
      treeNode.label = "MemberOfObject";
      const [object, member] = node.MemberOfObject;
      treeNode.attrs.MemberName = member;
      treeNode.children = node2tree([object]);
    } else if (node.MemberOfPointer) {
      treeNode.label = "MemberOfPointer";
      const [pointer, member] = node.MemberOfPointer;
      treeNode.attrs.MemberName = member;
      treeNode.children = node2tree([pointer]);
    } else if (node.ArraySubscript) {
      treeNode.label = "ArraySubscript";
      const [array, subscript] = node.ArraySubscript;
      treeNode.children = node2tree([array, subscript]);
    } else if (node.Identifier) {
      treeNode.label = "Identifier";
      const name = node.Identifier;
      treeNode.attrs.name = name;
    } else if (node.IntegerConstant) {
      treeNode.label = "IntegerConstant";
      const value = node.IntegerConstant;
      treeNode.attrs.value = value;
    } else if (node.UnsignedIntegerConstant) {
      treeNode.label = "UnsignedIntegerConstant";
      const value = node.UnsignedIntegerConstant;
      treeNode.attrs.value = value;
    } else if (node.LongConstant) {
      treeNode.label = "LongConstant";
      const value = node.LongConstant;
      treeNode.attrs.value = value;
    } else if (node.LongLongConstant) {
      treeNode.label = "LongLongConstant";
      const value = node.LongLongConstant;
      treeNode.attrs.value = value;
    } else if (node.FloatConstant) {
      treeNode.label = "FloatConstant";
      const value = node.FloatConstant;
      treeNode.attrs.value = value;
    } else if (node.DoubleConstant) {
      treeNode.label = "DoubleConstant";
      const value = node.DoubleConstant;
      treeNode.attrs.value = value;
    } else if (node.UnsignedLongConstant) {
      treeNode.label = "UnsignedLongConstant";
      const value = node.UnsignedLongConstant;
      treeNode.attrs.value = value;
    } else if (node.UnsignedLongLongConstant) {
      treeNode.label = "UnsignedLongLongConstant";
      const value = node.UnsignedLongLongConstant;
      treeNode.attrs.value = value;
    } else if (node.CharacterConstant) {
      treeNode.label = "CharacterConstant";
      const value = node.CharacterConstant;
      treeNode.attrs.value = value;
    } else if (node.StringLiteral) {
      treeNode.label = "StringLiteral";
      const value = node.StringLiteral;
      treeNode.attrs.value = value;
    } else if (node.Empty) {
      treeNode.label = "Empty";
    }

    nodeList.push(treeNode);
  }
  return nodeList;
}
