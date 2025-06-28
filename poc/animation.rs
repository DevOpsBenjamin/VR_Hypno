// Minimal Rust POC: convert editor JSON to keyframes JSON
// (Assume serde_json, serde, and anyhow are in Cargo.toml)

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::fs;

#[derive(Debug, Deserialize)]
struct EditorObject {
    id: String,
    #[serde(rename = "type")]
    obj_type: String,
    wireframe: bool,
    timeline: Vec<TimelineAction>,
}

#[derive(Debug, Deserialize)]
struct TimelineAction {
    start: f32,
    end: f32,
    action: String,
    #[serde(default)]
    axis: Option<String>,
    #[serde(default)]
    from: Option<serde_json::Value>,
    #[serde(default)]
    to: Option<serde_json::Value>,
}

#[derive(Debug, Serialize)]
struct OutputObject {
    id: String,
    frames: Vec<Frame>,
}

#[derive(Debug, Serialize)]
struct Frame {
    time: f32,
    visible: bool,
    rotation: f32,
    position: [f32; 3],
}

fn main() -> Result<()> {
    let input = fs::read_to_string("poc/editor_input.json")?;
    let data: serde_json::Value = serde_json::from_str(&input)?;
    let objects: Vec<EditorObject> = serde_json::from_value(data["objects"].clone())?;
    let mut out_objects = Vec::new();
    for obj in objects {
        let mut frames = Vec::new();
        let mut last_rot = 0.0;
        let mut last_pos = [0.0, 0.0, 0.0];
        for action in &obj.timeline {
            let step = 1.0; // 1s per frame for demo
            let mut t = action.start;
            while t <= action.end {
                match action.action.as_str() {
                    "rotate" => {
                        let from = action.from.as_ref().unwrap().as_f64().unwrap() as f32;
                        let to = action.to.as_ref().unwrap().as_f64().unwrap() as f32;
                        let frac = (t - action.start) / (action.end - action.start);
                        let rot = from + frac * (to - from);
                        last_rot = rot;
                        frames.push(Frame {
                            time: t,
                            visible: true,
                            rotation: rot,
                            position: last_pos,
                        });
                    }
                    "move" => {
                        let from: Vec<f32> = action.from.as_ref().unwrap().as_array().unwrap().iter().map(|v| v.as_f64().unwrap() as f32).collect();
                        let to: Vec<f32> = action.to.as_ref().unwrap().as_array().unwrap().iter().map(|v| v.as_f64().unwrap() as f32).collect();
                        let frac = (t - action.start) / (action.end - action.start);
                        let pos = [
                            from[0] + frac * (to[0] - from[0]),
                            from[1] + frac * (to[1] - from[1]),
                            from[2] + frac * (to[2] - from[2]),
                        ];
                        last_pos = pos;
                        frames.push(Frame {
                            time: t,
                            visible: true,
                            rotation: last_rot,
                            position: pos,
                        });
                    }
                    _ => {}
                }
                t += step;
            }
        }
        out_objects.push(OutputObject {
            id: obj.id,
            frames,
        });
    }
    let output = serde_json::json!({ "objects": out_objects });
    fs::write("poc/keyframes_output.json", serde_json::to_string_pretty(&output)?)?;
    Ok(())
}
