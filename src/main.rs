mod color;
mod dithering;
mod image_handling;
mod resizing;

use color::Color;

use dithering::{DitheringKernel, PALETTE16};

use anyhow::{anyhow, Result};
use image_handling::Image;
use std::collections::VecDeque;
use std::env;

trait PipelineStage {
    fn run(&self, input: Option<&Image>) -> Result<Option<Image>>;
}

struct LoadStage {
    path: String,
}
impl PipelineStage for LoadStage {
    fn run(&self, input: Option<&Image>) -> Result<Option<Image>> {
        Ok(Some(image_handling::load_image_from_file(&self.path)?))
    }
}
struct LanczosStage {
    new_width: usize,
    new_height: usize,
    a: f64,
}
impl PipelineStage for LanczosStage {
    fn run(&self, input: Option<&Image>) -> Result<Option<Image>> {
        if let Some(image) = input {
            Ok(Some(resizing::resize_lanczos(
                image,
                self.new_width,
                self.new_height,
                self.a,
            )?))
        } else {
            Err(anyhow!("Can't run Lanczos stage without input image"))
        }
    }
}
struct DitheringStage {
    palette: Vec<Color>,
    kernel: DitheringKernel,
}
impl PipelineStage for DitheringStage {
    fn run(&self, input: Option<&Image>) -> Result<Option<Image>> {
        if let Some(image) = input {
            Ok(Some(dithering::dither_image(
                image,
                &self.kernel,
                &self.palette,
            )))
        } else {
            Err(anyhow!("Can't run dithering stage without input image"))
        }
    }
}

struct SaveStage {
    path: String,
}
impl PipelineStage for SaveStage {
    fn run(&self, input: Option<&Image>) -> Result<Option<Image>> {
        if let Some(image) = input {
            image_handling::save_image_to_file(image, &self.path)?;
            Ok(None)
        } else {
            Err(anyhow!("Can't run save stage without input image"))
        }
    }
}

enum Stage {
    Load(LoadStage),
    Dithering(DitheringStage),
    Lanczos(LanczosStage),
    Save(SaveStage),
}

impl Stage {
    pub fn consume_args_and_create(args: &mut VecDeque<String>) -> Result<Self> {
        if args.len() > 0 {
            match args[0].as_str() {
                "--load" => {
                    args.pop_front();
                    if let Some(path) = args.pop_front() {
                        Ok(Stage::Load(LoadStage { path: path.clone() }))
                    } else {
                        Err(anyhow!("Can't parse path to load"))
                    }
                }
                "--lanczos" => {
                    args.pop_front();
                    if let Some(width) = args.pop_front() {
                        if let Some(height) = args.pop_front() {
                            Ok(Stage::Lanczos(LanczosStage {
                                new_width: width.parse::<usize>()?,
                                new_height: height.parse::<usize>()?,
                                a: 3f64,
                            }))
                        } else {
                            Err(anyhow!("Can't parse height to resize to"))
                        }
                    } else {
                        Err(anyhow!("Can't parse width to resize to"))
                    }
                }
                "--dither" => {
                    args.pop_front();

                    Ok(Stage::Dithering(DitheringStage {
                        palette: Vec::from(PALETTE16),
                        kernel: DitheringKernel::floyd_steinberg(),
                    }))
                }
                "--save" => {
                    args.pop_front();
                    if let Some(path) = args.pop_front() {
                        Ok(Stage::Save(SaveStage { path: path.clone() }))
                    } else {
                        Err(anyhow!("Can't parse path to save"))
                    }
                }
                _ => Err(anyhow!(
                    "Unknown pipeline stage from commandline {arg}",
                    arg = args[0]
                )),
            }
        } else {
            Err(anyhow!("Can't consume argument from empty array"))
        }
    }

    pub fn run(&self, image: Option<&Image>) -> Result<Option<Image>> {
        match self {
            Stage::Load(ref stage) => stage.run(image),
            Stage::Dithering(ref stage) => stage.run(image),
            Stage::Lanczos(ref stage) => stage.run(image),
            Stage::Save(ref stage) => stage.run(image),
        }
    }
}

struct Pipeline {
    stages: Vec<Stage>,
}

impl Pipeline {
    pub fn from_args(args: &mut VecDeque<String>) -> Result<Self> {
        let mut stages = vec![];
        while args.len() > 0 {
            stages.push(Stage::consume_args_and_create(args)?);
        }

        Ok(Pipeline { stages })
    }

    pub fn run(&self) -> Result<()> {
        let mut input: Option<&Image> = None;
        let mut _output: Option<Image> = None;
        for stage in &self.stages {
            _output = stage.run(input)?;
            input = _output.as_ref();
        }
        Ok(())
    }
}

fn main() -> Result<()> {
    let mut args = env::args().collect::<VecDeque<String>>();
    let command = args.pop_front().unwrap();

    if args.len() < 2 {
        eprintln!("Usage:");
        eprintln!("  {command} <pipeline-stage...>");
        eprintln!("");
        eprintln!("Stages:");
        eprintln!("  --load <filename>");
        eprintln!("  --lanczos <width> <height>");
        eprintln!("  --dither");
        eprintln!("  --save <filename>");
        std::process::exit(1);
    }

    let pipeline = Pipeline::from_args(&mut args)?;
    pipeline.run()?;

    Ok(())
}
