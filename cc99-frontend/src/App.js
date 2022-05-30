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
import { useState, useEffect } from "react";
import Editor from "./components/AceEditor";
import { ExampleCode } from "./data/example";
import Ast2Vis from "./utils/AST2Vis";
import init, { compile_result } from "cc99";
import AntVG6 from "./components/AntVG6";
import Logger from "./utils/logger";
import axios from "axios";
import Ansi from "ansi-to-react";

const { Panel } = Collapse;
const { TextArea } = Input;
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
    Logger.Info("Please compile the code before running!")
  );
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
  const appendOutput = (data) => {
    setOutput(`${output}\n${data}`);
  };

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
        setCompileStatus("服务器相关错误，未编译成功");
        notification.error({
          duration: 5,
          description: "服务器相关错误",
          message: result.data.msg,
        });
        return;
      }
      //查看编译是否成功，根据exitCode进行判断
      if (result.data.data.exitCode !== 0) {
        setCompileStatus("未编译成功");
        setExitCode(result.data.data.exitCode);
        setStdout(result.data.data.stdout);
        setStderr(result.data.data.stderr);
        return;
      }
      let file = result.data.data.file;
      if (!file) {
        setCompileStatus("编译成功，但是没有生成文件");
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
        setCompileStatus("服务器相关错误，编译成功，执行出现错误");
        notification.error({
          duration: 5,
          description: "服务器相关错误，编译成功，执行出现错误",
          message: result.data.msg,
        });
        return;
      }
      if (res.data.data.exitCode !== 0) {
        setCompileStatus("编译成功，执行出现错误");
      } else {
        setCompileStatus("编译成功，执行成功");
      }
      setStdout(res.data.data.stdout);
      setStderr(res.data.data.stderr);
      setExitCode(res.data.data.exitCode);
    } catch (e) {
      notification.error({
        duration: 5,
        description: "未知错误",
        message: e,
      });
    }
  };

  const compile = () => {
    let data = JSON.parse(compile_result(code));
    console.log(data);
    if (!data["error"]) {
      setAst(data["ast"]);
      setVisAst(Ast2Vis(data["ast"]));

      message.success("编译成功!");
      appendOutput(Logger.Info("compile success!"));
    } else {
      notification.error({
        message: "编译失败",
        description: data["message"],
        duration: 5,
      });
      appendOutput("compile error!\n" + Logger.Error(data["message"]));
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
            is the final project of ZJU Compiler Principle course, made by TO/GA, RalXYZ and
            Raynor. NOT{" "}
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
