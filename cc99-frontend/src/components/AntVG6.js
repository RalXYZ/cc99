import React, { useEffect, useRef, useState } from "react";
import ReactDOM from "react-dom";
import G6 from "@antv/g6";

//TODO 格式化数据用来展示
export default function AntvTree(props) {
  const ref = useRef(null);
  const [graph, setGraph] = useState(null);

  useEffect(() => {
    const { width, height } = ref.current.getBoundingClientRect();
    console.log(width, height);
    if (!graph) {
      const tooltip = new G6.Tooltip({
        offsetX: 10,
        offsetY: 20,
        getContent(e) {
          const outDiv = document.createElement("div");
          outDiv.style.width = "180px";
          let content = `<h4><b>${e.item.getModel().label}</b></h4>`;

          //   if (e.item.getModel().attrs) {
          //     const attrs = e.item.getModel().attrs;
          //     content += `<ul>`;
          //     for (const attr in attrs) {
          //       content += `<li><b> - ${attr}:</b> ${attrs[attr]}</li>`;
          //     }
          //     content += `</ul>`;
          //   }
          outDiv.innerHTML = content;
          return outDiv;
        },
        itemTypes: ["node"],
      });

      let tmpGraph = new G6.TreeGraph({
        container: ReactDOM.findDOMNode(ref.current),
        width: width,
        height: height,
        linkCenter: true,
        modes: {
          default: [
            {
              type: "collapse-expand",
              onChange: function onChange(item, collapsed) {
                const data = item.getModel();
                data.collapsed = collapsed;
                return true;
              },
            },
            "drag-canvas",
            "zoom-canvas",
          ],
        },
        defaultNode: {
          size: 26,
          anchorPoints: [
            [0, 0.5],
            [1, 0.5],
          ],
        },
        defaultEdge: {
          type: "cubic-vertical",
        },
        layout: {
          type: "compactBox",
          direction: "TB",
          getId: function getId(d) {
            return d.id;
          },
          getHeight: function getHeight() {
            return 16;
          },
          getWidth: function getWidth() {
            return 16;
          },
          getVGap: function getVGap() {
            return 80;
          },
          getHGap: function getHGap() {
            return 50;
          },
        },
        plugins: [tooltip],
      });

      tmpGraph.node((node) => {
        let position = "right";
        let rotate = 0;
        if (!node.children) {
          position = "bottom";
          rotate = 0;
          // rotate = Math.PI / 2;
        }
        return {
          style: {
            fill: "#ECFDF5",
            stroke: "#34D399",
          },
          label: node.label,
          labelCfg: {
            position,
            offset: 5,
            style: {
              rotate,
              textAlign: "start",
            },
          },
        };
      });
      tmpGraph.data(props.data);
      tmpGraph.render();
      tmpGraph.fitView();
      setGraph(tmpGraph);
    } else {
      graph.data(props.data);
      graph.render();
      graph.fitView();
    }
  }, [graph, props.data]);

  return <div ref={ref} style={{ width: "100%", height: "100%" }}></div>;
}
