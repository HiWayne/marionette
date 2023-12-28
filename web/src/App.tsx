import { useEffect, useRef, useState } from "react";
import * as flvjs from "flv.js";
import { Form, Input, Button } from "antd";

function App() {
  const [uniqueNumber, setUniqueNumber] = useState("");

  const videoRef = useRef(null);

  useEffect(() => {
    // const socket = new WebSocket("ws://0.0.0.0:10000");
    // socket.addEventListener("open", () => {
    //   socket.send(JSON.stringify({ eventName: "events", payload: "hello" }));
    // });
    // // Connection opened
    // socket.addEventListener("events", function (event: any) {
    //   console.log("Message from events ", event.data);
    // });
    // // Listen for messages
    // socket.addEventListener("identity", function (event: any) {
    //   console.log("Message from identity ", event.data);
    // });
    // return () => {
    //   socket.close();
    // };
  }, []);

  return (
    <div className="flex flex-col justify-start items-center p-[12px]">
      <video
        className="border-[1px] border-[#aaa] border-solid w-full"
        ref={videoRef}
      />
      <div className="mt-[20px]">
        <Form className="flex flex-col justify-start items-center">
          <Form.Item label="连接号">
            <Input
              className="w-[120px]"
              placeholder="输入房间号"
              value={uniqueNumber}
              onChange={(e) => {
                setUniqueNumber(e.target.value);
              }}
            />
          </Form.Item>
          <Button
            onClick={() => {
              if (videoRef.current) {
                const flvPlayer = (flvjs as any).createPlayer({
                  type: "flv",
                  url: `http://192.169.170.127:8000/live/${uniqueNumber}.flv`,
                });
                flvPlayer.attachMediaElement(videoRef.current);
                flvPlayer.load();
                flvPlayer.play();
              }
            }}
          >
            开启远程屏幕
          </Button>
        </Form>
      </div>
    </div>
  );
}

export default App;
