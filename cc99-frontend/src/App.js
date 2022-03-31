import "./App.css";
import { Layout } from "antd";
import { Card, Select, Button } from "antd";
import ResizePanel from "react-resize-panel";
import { useState, useEffect } from "react";
import Editor from "./components/AceEditor";
import { ExampleCode } from "./data/example";

import { Typography, Divider } from "antd";
const { Title, Text } = Typography;

const { Header, Footer, Content } = Layout;
const { Option } = Select;
function App() {
  const [code, setCode] = useState(ExampleCode[0].code);

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
              extra={<a href="#">More</a>}
              headStyle={{ fontWeight: "bold", fontSize: 22 }}
              style={{
                flexGrow: 1,
              }}
            >
              <p>Card content</p>
              <p>Card content</p>
              <p>Card content</p>
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
