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
    } else if (node.Statement) {
      treeNode.label = "Statement";
    }

    nodeList.push(treeNode);
  }
  return nodeList;
}
