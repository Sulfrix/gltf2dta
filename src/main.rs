use std::fmt::Debug;

use arson::{stdlib::stdio::print, Context, Node, NodeCommand};
use clap::Parser;
use gltf::mesh::util::{ReadIndices, ReadTexCoords};

#[derive(Parser, Debug)]
struct Args {
    #[arg(name = "input file")]
    input_path: String
}

fn cmd_new_mesh(name: &str, context: &mut Context) -> NodeCommand {
    let mut cmd = NodeCommand::new();
    cmd.push(context.add_symbol("new"));
    cmd.push(context.add_symbol("Mesh"));
    cmd.push(context.add_symbol(name));
    cmd
}

fn cmd_set_vert_pos(name: &str, context: &mut Context, index: u32, pos: [f32; 3]) -> NodeCommand {
    let mut cmd = NodeCommand::new();
    cmd.push(context.add_symbol(name));
    cmd.push(context.add_symbol("set_vert_pos"));
    cmd.push(index);
    cmd.push(pos[0]);
    cmd.push(pos[1]);
    cmd.push(pos[2]);
    cmd
}

fn cmd_set_vert_norm(name: &str, context: &mut Context, index: u32, norm: [f32; 3]) -> NodeCommand {
    let mut cmd = NodeCommand::new();
    cmd.push(context.add_symbol(name));
    cmd.push(context.add_symbol("set_vert_norm"));
    cmd.push(index);
    cmd.push(norm[0]);
    cmd.push(norm[1]);
    cmd.push(norm[2]);
    cmd
}

fn cmd_set_vert_uv(name: &str, context: &mut Context, index: u32, norm: [f32; 2]) -> NodeCommand {
    let mut cmd = NodeCommand::new();
    cmd.push(context.add_symbol(name));
    cmd.push(context.add_symbol("set_vert_uv"));
    cmd.push(index);
    cmd.push(norm[0]);
    cmd.push(norm[1]);
    cmd
}

fn cmd_set_face(name: &str, context: &mut Context, index: u32, indices: [u32; 3]) -> NodeCommand {
    let mut cmd = NodeCommand::new();
    cmd.push(context.add_symbol(name));
    cmd.push(context.add_symbol("set_face"));
    cmd.push(index);
    cmd.push(indices[0]);
    cmd.push(indices[1]);
    cmd.push(indices[2]);
    cmd
}

fn cmd_set_num_verts(name: &str, context: &mut Context, verts: u32) -> NodeCommand {
    let mut cmd = NodeCommand::new();
    cmd.push(context.add_symbol(name));
    cmd.push(context.add_symbol("set"));
    cmd.push(context.add_symbol("num_verts"));
    cmd.push(verts);
    cmd
}

fn cmd_set_num_faces(name: &str, context: &mut Context, faces: u32) -> NodeCommand {
    let mut cmd = NodeCommand::new();
    cmd.push(context.add_symbol(name));
    cmd.push(context.add_symbol("set"));
    cmd.push(context.add_symbol("num_faces"));
    cmd.push(faces);
    cmd
}

fn main() {
    let args = Args::parse();

    let (gltf, buffers, _) = gltf::import(args.input_path).unwrap();

    for scene in gltf.scenes() {
        for node in scene.nodes() {
            if let Some(mesh) = node.mesh() {
                for primitive in mesh.primitives() {
                    let mut positions: Vec<[f32; 3]> = Vec::new();
                    let mut normals: Vec<[f32; 3]> = Vec::new();
                    let mut uvs: Vec<[f32; 2]> = Vec::new();
                    let mut indices: Vec<[u32; 3]> = Vec::new();

                    //println!("; Primitive #{}", primitive.index());
                    let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
                    if let Some(iter) = reader.read_positions() {
                        for vertex_position in iter {
                            positions.push(vertex_position);
                        }
                    }
                    if let Some(iter) = reader.read_normals() {
                        for vertex_normal in iter {
                            normals.push(vertex_normal);
                        }
                    }
                    if let Some(ReadTexCoords::F32(iter)) = reader.read_tex_coords(0) {
                        for tex_coord in iter {
                            uvs.push(tex_coord);
                        }
                    }
                    if let Some(ReadIndices::U16(iter)) = reader.read_indices() {
                        let mut cur_index: [u32; 3] = [0, 0, 0];
                        let mut ind_ptr: usize = 0;
                        for index in iter {
                            cur_index[ind_ptr] = index.try_into().unwrap();
                            ind_ptr += 1;
                            if (ind_ptr == 3) {
                                ind_ptr = 0;
                                indices.push(cur_index);
                            }
                        }
                    }

                    let mut name_string = String::from(mesh.name().unwrap());
                    if !name_string.ends_with(".mesh") {
                        name_string.push_str(".mesh");
                    }

                    let name = name_string.as_str();

                    let mut ctx = Context::new();

                    println!("{}", cmd_new_mesh(name, &mut ctx));

                    println!("{}", cmd_set_num_verts(name, &mut ctx, positions.len().try_into().unwrap()));
                    println!("{}", cmd_set_num_faces(name, &mut ctx, indices.len().try_into().unwrap()));
                    let mut i = 0;
                    for vert in positions {
                        println!("{}", cmd_set_vert_pos(name, &mut ctx, i, vert));
                        i += 1;
                    }
                    i = 0;
                    for vert in normals {
                        println!("{}", cmd_set_vert_norm(name, &mut ctx, i, vert));
                        i += 1;
                    }
                    i = 0;
                    for vert in uvs {
                        println!("{}", cmd_set_vert_uv(name, &mut ctx, i, vert));
                        i += 1;
                    }
                }
            }
        }
    }
}
