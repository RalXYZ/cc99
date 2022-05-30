import "./App.css";
import {
  Card,
  Select,
  Button,
  message,
  Layout,
  notification,
  Input,
  Collapse,
} from "antd";

import ResizePanel from "react-resize-panel";
import { useState } from "react";
import Editor from "./components/AceEditor";
import { ExampleCode } from "./data/example";
import Ast2Vis from "./utils/AST2Vis";
import AntVG6 from "./components/AntVG6";
import axios from "axios";
import Ansi from "ansi-to-react";

const { Panel } = Collapse;
const { TextArea } = Input;
const { Header, Footer, Content } = Layout;
const { Option } = Select;
function App() {
  const [code, setCode] = useState(ExampleCode[0].code);
  const [visAst, setVisAst] = useState({ id: "0", label: "CC99" });
  const [stdin, setStdin] = useState("");
  const [compileOptions, setCompileOptions] = useState("");
  const [execArgs, setExecArgs] = useState("");

  const [stdout, setStdout] = useState("");
  const [stderr, setStderr] = useState("");
  const [exitCode, setExitCode] = useState(0);
  const [compileStatus, setCompileStatus] = useState("");

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

  const onClickRunCode = async () => {
    try {
      setStdout("");
      setStderr("");
      setExitCode("");
      setCompileStatus("");
      let result = await axios("/api/gen", {
        method: "post",
        headers: {
          "Content-Type": "application/json",
        },
        data: {
          code,
          compileOptions,
        },
      });

      if (result.data.st !== 0) {
        setCompileStatus("Server-related errors, did not compile successfully");
        notification.error({
          duration: 5,
          description: "Server related errors",
          message: result.data.msg,
        });
        return;
      }
      //查看编译是否成功，根据exitCode进行判断
      if (result.data.data.exitCode !== 0) {
        setCompileStatus("not compiled successfully");
        setExitCode(result.data.data.exitCode);
        setStdout(result.data.data.stdout);
        setStderr(result.data.data.stderr);
        return;
      }
      let file = result.data.data.file;
      if (!file) {
        setCompileStatus("Compilation succeeds, but no files are generated");
        setExitCode(result.data.data.exitCode);
        setStdout(result.data.data.stdout);
        setStderr(result.data.data.stderr);
        return;
      }
      let res = await axios("/api/run", {
        method: "post",
        headers: {
          "Content-Type": "application/json",
        },
        data: {
          file,
          execArgs,
          stdin,
        },
      });
      if (res.data.st !== 0) {
        setCompileStatus(
          "Server-related errors, compilation is successful, execution errors occur"
        );
        notification.error({
          duration: 5,
          description:
            "Server-related errors, compilation is successful, execution errors occur",
          message: result.data.msg,
        });
        return;
      }
      if (res.data.data.exitCode !== 0) {
        setCompileStatus("Compilation succeeded, execution error occurred");
      } else {
        setCompileStatus("Compilation succeeded, execution succeeded");
      }
      setStdout(res.data.data.stdout);
      setStderr(res.data.data.stderr);
      setExitCode(res.data.data.exitCode);
      await compile();
    } catch (e) {
      notification.error({
        duration: 5,
        description: "Unknown Error!",
        message: e,
      });
    }
  };

  const compile = async () => {
    try {
      let res = await axios("/api/visual", {
        method: "post",
        headers: {
          "Content-Type": "application/json",
        },
        data: {
          code,
        },
      });
      if (res.data.st !== 0) {
        notification.error({
          duration: 5,
          description: "Server related errors",
          message: res.data.msg,
        });
        return;
      }
      let data = JSON.parse(res.data.data.res);
      console.log(data);
      if (!data["error"]) {
        setVisAst(Ast2Vis(data["ast"]));

        message.success("Compile Success!");
      } else {
        notification.error({
          message: "Compile Error",
          description: data["message"],
          duration: 5,
        });
      }
    } catch (e) {
      notification.error({
        duration: 5,
        description: "Server related errors",
        message: e,
      });
    }
    // console.log(JSON.stringify(data["ast"], null, "\t"));
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
                  Visual!
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
                title="Compiler And Run"
                extra={
                  <Button type="primary" onClick={onClickRunCode}>
                    Run!
                  </Button>
                }
                headStyle={{ fontWeight: "bold", fontSize: 22 }}
                bodyStyle={{
                  flexGrow: 1,
                  overflowY: "auto",
                  paddingLeft: 2,
                  paddingRight: 2,
                }}
                style={{
                  width: "100%",
                  flexGrow: 1,
                  display: "flex",
                  flexDirection: "column",
                }}
              >
                <Input
                  placeholder="Compile options..."
                  onChange={(e) => setCompileOptions(e.target.value)}
                />
                <Input
                  placeholder="Execution arguments..."
                  onChange={(e) => setExecArgs(e.target.value)}
                />
                <TextArea
                  autoSize={{ minRows: 2, maxRows: 5 }}
                  rows={2}
                  placeholder="Execution stdin..."
                  onChange={(e) => setStdin(e.target.value)}
                />
                <div>Compile Status: {compileStatus}</div>
                <div>Return Code:{exitCode}</div>
                <Collapse
                  bordered={false}
                  defaultActiveKey={["stdout", "stderr"]}
                >
                  <Panel header="Stdout" key="stdout">
                    <pre>
                      <Ansi>{stdout}</Ansi>
                    </pre>
                  </Panel>
                  <Panel header="Stderr" key="stderr">
                    <pre>
                      <Ansi>{stderr}</Ansi>
                    </pre>
                  </Panel>
                </Collapse>
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
              cc99{" "}
            </a>{" "}
            is the final project of ZJU Compiler Principle course, made by
            TO/GA, RalXYZ and Raynor. NOT{" "}
            <a
              href={"https://cc98.org"}
              className="no-underline hover:underline"
            >
              cc98.org{" "}
            </a>{" "}
            !
          </div>
        </Footer>
      </Layout>
    </>
  );
}

export default App;
