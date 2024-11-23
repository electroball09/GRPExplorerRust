use super::*;
use crate::objects::{*, node_ids::ShaderNodeId, node_ids::ShaderNodeId::*};

mod node_id_editors;
use node_id_editors::*;

pub struct ShaderGraphEditor;

impl EditorImpl for ShaderGraphEditor {
    fn draw(&mut self, key: u32, ui: &mut egui::Ui, ectx: &mut EditorContext) {
        if let ObjectArchetype::ShaderGraph(shd) = &mut ectx.bf.object_table.get_mut(&key).unwrap().archetype {
            ui.label(format!("version: {:#06X}", shd.version));
            ui.label(format!("flags: {:#06X} {:#018b}", shd.flags, shd.flags));

            let mut i = 0;
            egui::ScrollArea::new([false, true]).auto_shrink([false, false]).show(ui, |ui| {
                for graph in &mut shd.graphs {
                    ui.collapsing(format!("{}", i), |ui| {
                        ui.label(format!("unk_01: {:#010X} {:#034b}", graph.unk_01, graph.unk_01));
                        ui.label(format!("unk_02: {:#010X} {:#034b}", graph.unk_02, graph.unk_02));
                        ui.label(format!("unk_03: {:#010X} {:#034b}", graph.unk_03, graph.unk_03));
                        ui.label(format!("unk_04: {:#010X} {:#034b}", graph.unk_04, graph.unk_04));
                        ui.label(format!("num_nodes: {}", graph.num_nodes));
                        ui.label(format!("unk_06: {:#010X}", graph.unk_06));
                        
                        let mut j = 0;
                        for node in &mut graph.nodes {
                            ui.collapsing(format!("{:#010X} {}", j, node.get_id()), |ui| {
                                ui.label(format!("unk_01: {:#010X} {:#034b}", node.unk_01, node.unk_01));
                                let old_node = std::mem::replace(&mut node.node, ShaderNodeId::Invalid);
                                node.node = draw_node_id_editor(old_node, ui, ectx.ctx);
                            });
                            j += 1;
                        }
                    });
                    i += 1;
                }
            });
        }
    }
}

fn draw_node_id_editor(node: ShaderNodeId, ui: &mut egui::Ui, ctx: &egui::Context) -> ShaderNodeId {
    match node {
        eSID_ADD(node) => draw_eSID_ADD(node, ui, ctx),
        eSID_AmbientCube(node) => draw_eSID_AmbientCube(node, ui, ctx),
        eSID_AmbientOcclusion(node) => draw_eSID_AmbientOcclusion(node, ui, ctx),
        eSID_Blender(node) => draw_eSID_Blender(node, ui, ctx),
        eSID_BlurTex(node) => draw_eSID_BlurTex(node, ui, ctx),
        eSID_Clamp(node) => draw_eSID_Clamp(node, ui, ctx),
        eSID_ColorSelector(node) => draw_eSID_ColorSelector(node, ui, ctx),
        eSID_ColorSelectorSmooth(node) => draw_eSID_ColorSelectorSmooth(node, ui, ctx),
        eSID_Combiner4D(node) => draw_eSID_Combiner4D(node, ui, ctx),
        eSID_Comment(node) => draw_eSID_Comment(node, ui, ctx),
        eSID_ConstantColor(node) => draw_eSID_ConstantColor(node, ui, ctx),
        eSID_ConstantMUL(node) => draw_eSID_ConstantMUL(node, ui, ctx),
        eSID_ConstantUVWQ(node) => draw_eSID_ConstantUVWQ(node, ui, ctx),
        eSID_ConstantVector(node) => draw_eSID_ConstantVector(node, ui, ctx),
        eSID_CustomCode(node) => draw_eSID_CustomCode(node, ui, ctx),
        eSID_CustomCodeVtx(node) => draw_eSID_CustomCodeVtx(node, ui, ctx),
        eSID_DepthAlpha(node) => draw_eSID_DepthAlpha(node, ui, ctx),
        eSID_DepthAlphaOpt(node) => draw_eSID_DepthAlphaOpt(node, ui, ctx),
        eSID_DepthTexture(node) => draw_eSID_DepthTexture(node, ui, ctx),
        eSID_DiffuseCube(node) => draw_eSID_DiffuseCube(node, ui, ctx),
        eSID_DiffuseMultiplier(node) => draw_eSID_DiffuseMultiplier(node, ui, ctx),
        eSID_DustFXSH(node) => draw_eSID_DustFXSH(node, ui, ctx),
        eSID_ElapseTime(node) => draw_eSID_ElapseTime(node, ui, ctx),
        eSID_FlatChrome(node) => draw_eSID_FlatChrome(node, ui, ctx),
        eSID_ForceNoPrepass(node) => draw_eSID_ForceNoPrepass(node, ui, ctx),
        eSID_ForceUltraSimpleShader(node) => draw_eSID_ForceUltraSimpleShader(node, ui, ctx),
        eSID_ForceVisualPrepass(node) => draw_eSID_ForceVisualPrepass(node, ui, ctx),
        eSID_HeatSelector(node) => draw_eSID_HeatSelector(node, ui, ctx),
        eSID_Invert(node) => draw_eSID_Invert(node, ui, ctx),
        eSID_LODBlender(node) => draw_eSID_LODBlender(node, ui, ctx),
        eSID_LODMUL(node) => draw_eSID_LODMUL(node, ui, ctx),
        eSID_LightCurve_Back(node) => draw_eSID_LightCurve_Back(node, ui, ctx),
        eSID_LinearFresnel(node) => draw_eSID_LinearFresnel(node, ui, ctx),
        eSID_Luminance(node) => draw_eSID_Luminance(node, ui, ctx),
        eSID_MUL(node) => draw_eSID_MUL(node, ui, ctx),
        eSID_MUL2X(node) => draw_eSID_MUL2X(node, ui, ctx),
        eSID_MainOutput(node) => draw_eSID_MainOutput(node, ui, ctx),
        eSID_MaterialColor_Diffuse(node) => draw_eSID_MaterialColor_Diffuse(node, ui, ctx),
        eSID_MaterialColor_Emissive(node) => draw_eSID_MaterialColor_Emissive(node, ui, ctx),
        eSID_MaterialColor_EmissiveVTX(node) => draw_eSID_MaterialColor_EmissiveVTX(node, ui, ctx),
        eSID_MaterialColor_Specular(node) => draw_eSID_MaterialColor_Specular(node, ui, ctx),
        eSID_Normal(node) => draw_eSID_Normal(node, ui, ctx),
        eSID_Normalize3D(node) => draw_eSID_Normalize3D(node, ui, ctx),
        eSID_OffsetBump(node) => draw_eSID_OffsetBump(node, ui, ctx),
        eSID_PixelAverageColor(node) => draw_eSID_PixelAverageColor(node, ui, ctx),
        eSID_PixelColor_Misc(node) => draw_eSID_PixelColor_Misc(node, ui, ctx),
        eSID_PixelColor_Misc2(node) => draw_eSID_PixelColor_Misc2(node, ui, ctx),
        eSID_PixelSH(node) => draw_eSID_PixelSH(node, ui, ctx),
        eSID_PixelUVBoxAnimBlend(node) => draw_eSID_PixelUVBoxAnimBlend(node, ui, ctx),
        eSID_PixelViewToWorld(node) => draw_eSID_PixelViewToWorld(node, ui, ctx),
        eSID_Position(node) => draw_eSID_Position(node, ui, ctx),
        eSID_PowFresnel(node) => draw_eSID_PowFresnel(node, ui, ctx),
        eSID_Power(node) => draw_eSID_Power(node, ui, ctx),
        eSID_PulseWave(node) => draw_eSID_PulseWave(node, ui, ctx),
        eSID_RGB2UV(node) => draw_eSID_RGB2UV(node, ui, ctx),
        eSID_RGB_Ramp(node) => draw_eSID_RGB_Ramp(node, ui, ctx),
        eSID_RefracTex(node) => draw_eSID_RefracTex(node, ui, ctx),
        eSID_SUB(node) => draw_eSID_SUB(node, ui, ctx),
        eSID_SawWave(node) => draw_eSID_SawWave(node, ui, ctx),
        eSID_ShadowValue(node) => draw_eSID_ShadowValue(node, ui, ctx),
        eSID_SinusFX(node) => draw_eSID_SinusFX(node, ui, ctx),
        eSID_SpecularCubeMap(node) => draw_eSID_SpecularCubeMap(node, ui, ctx),
        eSID_SpecularGlossMultiplier(node) => draw_eSID_SpecularGlossMultiplier(node, ui, ctx),
        eSID_SpecularPowerMultiplier(node) => draw_eSID_SpecularPowerMultiplier(node, ui, ctx),
        eSID_Tangent2Screen(node) => draw_eSID_Tangent2Screen(node, ui, ctx),
        eSID_Tex2D(node) => draw_eSID_Tex2D(node, ui, ctx),
        eSID_Tex2DYUV(node) => draw_eSID_Tex2DYUV(node, ui, ctx),
        eSID_TexBump(node) => draw_eSID_TexBump(node, ui, ctx),
        eSID_TexBumpTangent(node) => draw_eSID_TexBumpTangent(node, ui, ctx),
        eSID_UV2RGB(node) => draw_eSID_UV2RGB(node, ui, ctx),
        eSID_UVRotate(node) => draw_eSID_UVRotate(node, ui, ctx),
        eSID_UVScroll(node) => draw_eSID_UVScroll(node, ui, ctx),
        eSID_UVScrollSpeedFactor(node) => draw_eSID_UVScrollSpeedFactor(node, ui, ctx),
        eSID_UVSource(node) => draw_eSID_UVSource(node, ui, ctx),
        eSID_UV_ADD(node) => draw_eSID_UV_ADD(node, ui, ctx),
        eSID_UV_Blender(node) => draw_eSID_UV_Blender(node, ui, ctx),
        eSID_UV_Combiner4D(node) => draw_eSID_UV_Combiner4D(node, ui, ctx),
        eSID_UV_ConstantMUL(node) => draw_eSID_UV_ConstantMUL(node, ui, ctx),
        eSID_UV_DUDV(node) => draw_eSID_UV_DUDV(node, ui, ctx),
        eSID_UV_MUL(node) => draw_eSID_UV_MUL(node, ui, ctx),
        eSID_UV_SUB(node) => draw_eSID_UV_SUB(node, ui, ctx),
        eSID_UntransformedNormal(node) => draw_eSID_UntransformedNormal(node, ui, ctx),
        eSID_VERTEX_UV_SUB(node) => draw_eSID_VERTEX_UV_SUB(node, ui, ctx),
        eSID_VertexColor(node) => draw_eSID_VertexColor(node, ui, ctx),
        eSID_VertexColorToPixel(node) => draw_eSID_VertexColorToPixel(node, ui, ctx),
        eSID_VertexColor_Misc1(node) => draw_eSID_VertexColor_Misc1(node, ui, ctx),
        eSID_VertexColor_Misc2(node) => draw_eSID_VertexColor_Misc2(node, ui, ctx),
        eSID_VertexCombiner4D(node) => draw_eSID_VertexCombiner4D(node, ui, ctx),
        eSID_VertexConstUVWQ(node) => draw_eSID_VertexConstUVWQ(node, ui, ctx),
        eSID_VertexConstantVector(node) => draw_eSID_VertexConstantVector(node, ui, ctx),
        eSID_VertexElapseTime(node) => draw_eSID_VertexElapseTime(node, ui, ctx),
        eSID_VertexInvert(node) => draw_eSID_VertexInvert(node, ui, ctx),
        eSID_VertexNormal(node) => draw_eSID_VertexNormal(node, ui, ctx),
        eSID_VertexPulseWave(node) => draw_eSID_VertexPulseWave(node, ui, ctx),
        eSID_VertexRGB2UV(node) => draw_eSID_VertexRGB2UV(node, ui, ctx),
        eSID_VertexUVRotate(node) => draw_eSID_VertexUVRotate(node, ui, ctx),
        eSID_VertexUVScroll(node) => draw_eSID_VertexUVScroll(node, ui, ctx),
        eSID_VertexUVSource(node) => draw_eSID_VertexUVSource(node, ui, ctx),
        eSID_VertexUVToPixelUV(node) => draw_eSID_VertexUVToPixelUV(node, ui, ctx),
        eSID_VertexUV_Combiner4D(node) => draw_eSID_VertexUV_Combiner4D(node, ui, ctx),
        eSID_VertexUntransformedNormal(node) => draw_eSID_VertexUntransformedNormal(node, ui, ctx),
        eSID_Vertex_UV_ADD(node) => draw_eSID_Vertex_UV_ADD(node, ui, ctx),
        eSID_Vertex_UV_Blender(node) => draw_eSID_Vertex_UV_Blender(node, ui, ctx),
        eSID_Vertex_UV_ConstantMUL(node) => draw_eSID_Vertex_UV_ConstantMUL(node, ui, ctx),
        eSID_Vertex_UV_MUL(node) => draw_eSID_Vertex_UV_MUL(node, ui, ctx),
        eSID_ViewPosition(node) => draw_eSID_ViewPosition(node, ui, ctx),
        eSID_VolumeAlphaEx(node) => draw_eSID_VolumeAlphaEx(node, ui, ctx),
        eSID_VolumeAmbLightIntensityOpt(node) => draw_eSID_VolumeAmbLightIntensityOpt(node, ui, ctx),
        eSID_VolumetricParticle(node) => draw_eSID_VolumetricParticle(node, ui, ctx),
        eSID_VtxWorldPosition(node) => draw_eSID_VtxWorldPosition(node, ui, ctx),
        eSID_WorldPosition(node) => draw_eSID_WorldPosition(node, ui, ctx),
        eSID_WorldSinusFX_VC(node) => draw_eSID_WorldSinusFX_VC(node, ui, ctx),
        Invalid => {
            ui.label("no node editor for this type yet!");
            ShaderNodeId::Invalid
        }
    }
}