import React from "react";
import AceEditor from "react-ace";
import "ace-builds/src-noconflict/mode-c_cpp";
import "ace-builds/src-noconflict/ext-language_tools";
import "ace-builds/src-noconflict/ext-searchbox";

import "ace-builds/src-noconflict/theme-github";

function Editor(props) {
  const { code, setCode } = props;

  return (
    <>
      <AceEditor
        value={code}
        onChange={setCode}
        mode="c_cpp"
        theme="xcode"
        fontSize={14}
        style={{
          height: "100%",
          overflow: "hidden",
          width: "100%",
          minHeight: 300,
          fontFamily: "Fira Code, Consolas, monospace",
        }}
        setOptions={{
          enableBasicAutocompletion: false, //关闭基本自动完成功能
          enableLiveAutocompletion: true, //启用实时自动完成功能
          enableSnippets: true,
          showLineNumbers: true,
          editorProps: { $blockScrolling: true },
          highlightActiveLine: true,
          tabSize: 2,
        }}
      ></AceEditor>
    </>
  );
}

export default Editor;
