import "./App.css";
import {
  Card,
  Select,
  Button,
  message,
  Layout,
  Typography,
  notification,
} from "antd";
import ResizePanel from "react-resize-panel";
import { useState, useEffect } from "react";
import Editor from "./components/AceEditor";
import { ExampleCode } from "./data/example";

import init, { compile_result } from "cc99";
import AntVG6 from "./components/AntVG6";
const { Text } = Typography;

const { Header, Footer, Content } = Layout;
const { Option } = Select;
function App() {
  useEffect(() => {
    init();
  }, []);
  const [code, setCode] = useState(ExampleCode[0].code);
  const [ast, setAst] = useState({ id: "0", label: "CC99" });
  const [visAst, setVisAst] = useState({ id: "0", label: "CC99" });
  const [output, setOutput] = useState(
    `[INFO] Compile the code before running!`
  );
  const codeSelector = (
    <Select
      style={{ width: 150 }}
      defaultValue={0}
      onChange={(e) => setCode(ExampleCode[e].code)}
    >
      {ExampleCode.map((e) => (
        <Option key={e.id} value={e.id}>
          {e.name}
        </Option>
      ))}
    </Select>
  );

  const onClickRunCode = () => {
    setOutput(`${output}<br>[INFO] 别点我!`);
  };

  const compile = () => {
    console.log(code);
    let data = JSON.parse(compile_result(code));
    if (!data["error"]) {
      setAst(data["ast"]);
      setVisAst(data["ast"]);
      message.success("编译成功!");
    } else {
      notification.error({
        message: "编译失败",
        description: data["error_message"],
        duration: null,
      });
    }
    console.log(data["ast"]);
    console.log(JSON.stringify(data["ast"], null, "\t"));
  };
  return (
    <>
      <Layout style={{ height: "100vh" }}>
        <Header className="App-header">CC99</Header>
        <Content>
          <div
            style={{
              display: "flex",
              height: "100%",
              width: "100%",
            }}
          >
            <ResizePanel direction="e" style={{ flexGrow: 1 }}>
              <Card
                title="Code"
                extra={codeSelector}
                headStyle={{ fontWeight: "bold", fontSize: 22 }}
                bodyStyle={{ flexGrow: 1, padding: 0, overflow: "hidden" }}
                style={{
                  width: "100%",
                  flexGrow: 1,
                  display: "flex",
                  flexDirection: "column",
                }}
              >
                <Editor code={code} setCode={setCode}></Editor>
              </Card>
            </ResizePanel>
            <Card
              title="Visualization"
              extra={
                <Button type="primary" onClick={compile}>
                  Compile!
                </Button>
              }
              headStyle={{ fontWeight: "bold", fontSize: 22 }}
              bodyStyle={{ flexGrow: 1, padding: 0, overflow: "hidden" }}
              style={{
                flexGrow: 1,
                display: "flex",
                flexDirection: "column",
              }}
            >
              <AntVG6 data={visAst} />
            </Card>

            <ResizePanel direction="w" style={{ flexGrow: 1 }}>
              <Card
                title="Result"
                extra={
                  <Button type="primary" onClick={onClickRunCode}>
                    Run!
                  </Button>
                }
                headStyle={{ fontWeight: "bold", fontSize: 22 }}
                style={{
                  width: "100%",
                  flexGrow: 1,
                }}
              >
                <Typography>
                  <Text
                    strong={false}
                    type="secondary"
                    style={{ fontSize: 16 }}
                  >
                    {output}
                  </Text>
                </Typography>
              </Card>
            </ResizePanel>
          </div>
        </Content>
        <Footer>
          <div style={{ textAlign: "center", fontSize: 17 }}>
            <a
              href={"https://github.com/RalXYZ/cc99"}
              className="no-underline hover:underline"
            >
              CC99{" "}
            </a>{" "}
            is the course project of ZJU Compilation, made by TO/GA,Ralph and
            Raynor. NOT{" "}
            <a
              href={"https://cc98.org"}
              className="no-underline hover:underline"
            >
              CC98.org{" "}
            </a>{" "}
            , we are Compile of C99!
          </div>
        </Footer>
      </Layout>
    </>
  );
}

export default App;
