#![allow(non_snake_case, dead_code, unused_variables, unused_mut)]

use crate::objects::node_ids::*;
use crate::egui as egui;


pub fn draw_eSID_ADD(mut node:eSID_ADD, ui: &mut egui::Ui, _ctx: &egui::Context) -> ShaderNodeId {

    ShaderNodeId::eSID_ADD(node)
}

pub fn draw_eSID_AmbientCube(mut node:eSID_AmbientCube, ui: &mut egui::Ui, _ctx: &egui::Context) -> ShaderNodeId {

    ShaderNodeId::eSID_AmbientCube(node)
}

pub fn draw_eSID_AmbientOcclusion(mut node:eSID_AmbientOcclusion, ui: &mut egui::Ui, _ctx: &egui::Context) -> ShaderNodeId {

    ShaderNodeId::eSID_AmbientOcclusion(node)
}

pub fn draw_eSID_Blender(mut node:eSID_Blender, ui: &mut egui::Ui, _ctx: &egui::Context) -> ShaderNodeId {

    ShaderNodeId::eSID_Blender(node)
}

pub fn draw_eSID_BlurTex(mut node:eSID_BlurTex, ui: &mut egui::Ui, _ctx: &egui::Context) -> ShaderNodeId {

    ShaderNodeId::eSID_BlurTex(node)
}

pub fn draw_eSID_Clamp(mut node:eSID_Clamp, ui: &mut egui::Ui, _ctx: &egui::Context) -> ShaderNodeId {

    ShaderNodeId::eSID_Clamp(node)
}

pub fn draw_eSID_ColorSelector(mut node:eSID_ColorSelector, ui: &mut egui::Ui, _ctx: &egui::Context) -> ShaderNodeId {

    ShaderNodeId::eSID_ColorSelector(node)
}

pub fn draw_eSID_ColorSelectorSmooth(mut node:eSID_ColorSelectorSmooth, ui: &mut egui::Ui, _ctx: &egui::Context) -> ShaderNodeId {

    ShaderNodeId::eSID_ColorSelectorSmooth(node)
}

pub fn draw_eSID_Combiner4D(mut node:eSID_Combiner4D, ui: &mut egui::Ui, _ctx: &egui::Context) -> ShaderNodeId {

    ShaderNodeId::eSID_Combiner4D(node)
}

pub fn draw_eSID_Comment(mut node:eSID_Comment, ui: &mut egui::Ui, _ctx: &egui::Context) -> ShaderNodeId {
    ui.centered_and_justified(|ui| {
        ui.text_edit_multiline(&mut node.comment);
    });
    
    ShaderNodeId::eSID_Comment(node)
}

pub fn draw_eSID_ConstantColor(mut node:eSID_ConstantColor, ui: &mut egui::Ui, _ctx: &egui::Context) -> ShaderNodeId {

    ShaderNodeId::eSID_ConstantColor(node)
}

pub fn draw_eSID_ConstantMUL(mut node:eSID_ConstantMUL, ui: &mut egui::Ui, _ctx: &egui::Context) -> ShaderNodeId {

    ShaderNodeId::eSID_ConstantMUL(node)
}

pub fn draw_eSID_ConstantUVWQ(mut node:eSID_ConstantUVWQ, ui: &mut egui::Ui, _ctx: &egui::Context) -> ShaderNodeId {

    ShaderNodeId::eSID_ConstantUVWQ(node)
}

pub fn draw_eSID_ConstantVector(mut node:eSID_ConstantVector, ui: &mut egui::Ui, _ctx: &egui::Context) -> ShaderNodeId {

    ShaderNodeId::eSID_ConstantVector(node)
}

pub fn draw_eSID_CustomCode(mut node:eSID_CustomCode, ui: &mut egui::Ui, _ctx: &egui::Context) -> ShaderNodeId {

    ShaderNodeId::eSID_CustomCode(node)
}

pub fn draw_eSID_CustomCodeVtx(mut node:eSID_CustomCodeVtx, ui: &mut egui::Ui, _ctx: &egui::Context) -> ShaderNodeId {

    ShaderNodeId::eSID_CustomCodeVtx(node)
}

pub fn draw_eSID_DepthAlpha(mut node:eSID_DepthAlpha, ui: &mut egui::Ui, _ctx: &egui::Context) -> ShaderNodeId {

    ShaderNodeId::eSID_DepthAlpha(node)
}

pub fn draw_eSID_DepthAlphaOpt(mut node:eSID_DepthAlphaOpt, ui: &mut egui::Ui, _ctx: &egui::Context) -> ShaderNodeId {

    ShaderNodeId::eSID_DepthAlphaOpt(node)
}

pub fn draw_eSID_DepthTexture(mut node:eSID_DepthTexture, ui: &mut egui::Ui, _ctx: &egui::Context) -> ShaderNodeId {

    ShaderNodeId::eSID_DepthTexture(node)
}

pub fn draw_eSID_DiffuseCube(mut node:eSID_DiffuseCube, ui: &mut egui::Ui, _ctx: &egui::Context) -> ShaderNodeId {

    ShaderNodeId::eSID_DiffuseCube(node)
}

pub fn draw_eSID_DiffuseMultiplier(mut node:eSID_DiffuseMultiplier, ui: &mut egui::Ui, _ctx: &egui::Context) -> ShaderNodeId {

    ShaderNodeId::eSID_DiffuseMultiplier(node)
}

pub fn draw_eSID_DustFXSH(mut node:eSID_DustFXSH, ui: &mut egui::Ui, _ctx: &egui::Context) -> ShaderNodeId {

    ShaderNodeId::eSID_DustFXSH(node)
}

pub fn draw_eSID_ElapseTime(mut node:eSID_ElapseTime, ui: &mut egui::Ui, _ctx: &egui::Context) -> ShaderNodeId {

    ShaderNodeId::eSID_ElapseTime(node)
}

pub fn draw_eSID_FlatChrome(mut node:eSID_FlatChrome, ui: &mut egui::Ui, _ctx: &egui::Context) -> ShaderNodeId {

    ShaderNodeId::eSID_FlatChrome(node)
}

pub fn draw_eSID_ForceNoPrepass(mut node:eSID_ForceNoPrepass, ui: &mut egui::Ui, _ctx: &egui::Context) -> ShaderNodeId {

    ShaderNodeId::eSID_ForceNoPrepass(node)
}

pub fn draw_eSID_ForceUltraSimpleShader(mut node:eSID_ForceUltraSimpleShader, ui: &mut egui::Ui, _ctx: &egui::Context) -> ShaderNodeId {

    ShaderNodeId::eSID_ForceUltraSimpleShader(node)
}

pub fn draw_eSID_ForceVisualPrepass(mut node:eSID_ForceVisualPrepass, ui: &mut egui::Ui, _ctx: &egui::Context) -> ShaderNodeId {

    ShaderNodeId::eSID_ForceVisualPrepass(node)
}

pub fn draw_eSID_HeatSelector(mut node:eSID_HeatSelector, ui: &mut egui::Ui, _ctx: &egui::Context) -> ShaderNodeId {

    ShaderNodeId::eSID_HeatSelector(node)
}

pub fn draw_eSID_Invert(mut node:eSID_Invert, ui: &mut egui::Ui, _ctx: &egui::Context) -> ShaderNodeId {

    ShaderNodeId::eSID_Invert(node)
}

pub fn draw_eSID_LODBlender(mut node:eSID_LODBlender, ui: &mut egui::Ui, _ctx: &egui::Context) -> ShaderNodeId {

    ShaderNodeId::eSID_LODBlender(node)
}

pub fn draw_eSID_LODMUL(mut node:eSID_LODMUL, ui: &mut egui::Ui, _ctx: &egui::Context) -> ShaderNodeId {

    ShaderNodeId::eSID_LODMUL(node)
}

pub fn draw_eSID_LightCurve_Back(mut node:eSID_LightCurve_Back, ui: &mut egui::Ui, _ctx: &egui::Context) -> ShaderNodeId {

    ShaderNodeId::eSID_LightCurve_Back(node)
}

pub fn draw_eSID_LinearFresnel(mut node:eSID_LinearFresnel, ui: &mut egui::Ui, _ctx: &egui::Context) -> ShaderNodeId {

    ShaderNodeId::eSID_LinearFresnel(node)
}

pub fn draw_eSID_Luminance(mut node:eSID_Luminance, ui: &mut egui::Ui, _ctx: &egui::Context) -> ShaderNodeId {

    ShaderNodeId::eSID_Luminance(node)
}

pub fn draw_eSID_MUL(mut node:eSID_MUL, ui: &mut egui::Ui, _ctx: &egui::Context) -> ShaderNodeId {

    ShaderNodeId::eSID_MUL(node)
}

pub fn draw_eSID_MUL2X(mut node:eSID_MUL2X, ui: &mut egui::Ui, _ctx: &egui::Context) -> ShaderNodeId {

    ShaderNodeId::eSID_MUL2X(node)
}

pub fn draw_eSID_MainOutput(mut node:eSID_MainOutput, ui: &mut egui::Ui, _ctx: &egui::Context) -> ShaderNodeId {

    ShaderNodeId::eSID_MainOutput(node)
}

pub fn draw_eSID_MaterialColor_Diffuse(mut node:eSID_MaterialColor_Diffuse, ui: &mut egui::Ui, _ctx: &egui::Context) -> ShaderNodeId {

    ShaderNodeId::eSID_MaterialColor_Diffuse(node)
}

pub fn draw_eSID_MaterialColor_Emissive(mut node:eSID_MaterialColor_Emissive, ui: &mut egui::Ui, _ctx: &egui::Context) -> ShaderNodeId {

    ShaderNodeId::eSID_MaterialColor_Emissive(node)
}

pub fn draw_eSID_MaterialColor_EmissiveVTX(mut node:eSID_MaterialColor_EmissiveVTX, ui: &mut egui::Ui, _ctx: &egui::Context) -> ShaderNodeId {

    ShaderNodeId::eSID_MaterialColor_EmissiveVTX(node)
}

pub fn draw_eSID_MaterialColor_Specular(mut node:eSID_MaterialColor_Specular, ui: &mut egui::Ui, _ctx: &egui::Context) -> ShaderNodeId {

    ShaderNodeId::eSID_MaterialColor_Specular(node)
}

pub fn draw_eSID_Normal(mut node:eSID_Normal, ui: &mut egui::Ui, _ctx: &egui::Context) -> ShaderNodeId {

    ShaderNodeId::eSID_Normal(node)
}

pub fn draw_eSID_Normalize3D(mut node:eSID_Normalize3D, ui: &mut egui::Ui, _ctx: &egui::Context) -> ShaderNodeId {

    ShaderNodeId::eSID_Normalize3D(node)
}

pub fn draw_eSID_OffsetBump(mut node:eSID_OffsetBump, ui: &mut egui::Ui, _ctx: &egui::Context) -> ShaderNodeId {

    ShaderNodeId::eSID_OffsetBump(node)
}

pub fn draw_eSID_PixelAverageColor(mut node:eSID_PixelAverageColor, ui: &mut egui::Ui, _ctx: &egui::Context) -> ShaderNodeId {

    ShaderNodeId::eSID_PixelAverageColor(node)
}

pub fn draw_eSID_PixelColor_Misc(mut node:eSID_PixelColor_Misc, ui: &mut egui::Ui, _ctx: &egui::Context) -> ShaderNodeId {

    ShaderNodeId::eSID_PixelColor_Misc(node)
}

pub fn draw_eSID_PixelColor_Misc2(mut node:eSID_PixelColor_Misc2, ui: &mut egui::Ui, _ctx: &egui::Context) -> ShaderNodeId {

    ShaderNodeId::eSID_PixelColor_Misc2(node)
}

pub fn draw_eSID_PixelSH(mut node:eSID_PixelSH, ui: &mut egui::Ui, _ctx: &egui::Context) -> ShaderNodeId {

    ShaderNodeId::eSID_PixelSH(node)
}

pub fn draw_eSID_PixelUVBoxAnimBlend(mut node:eSID_PixelUVBoxAnimBlend, ui: &mut egui::Ui, _ctx: &egui::Context) -> ShaderNodeId {

    ShaderNodeId::eSID_PixelUVBoxAnimBlend(node)
}

pub fn draw_eSID_PixelViewToWorld(mut node:eSID_PixelViewToWorld, ui: &mut egui::Ui, _ctx: &egui::Context) -> ShaderNodeId {

    ShaderNodeId::eSID_PixelViewToWorld(node)
}

pub fn draw_eSID_Position(mut node:eSID_Position, ui: &mut egui::Ui, _ctx: &egui::Context) -> ShaderNodeId {

    ShaderNodeId::eSID_Position(node)
}

pub fn draw_eSID_PowFresnel(mut node:eSID_PowFresnel, ui: &mut egui::Ui, _ctx: &egui::Context) -> ShaderNodeId {

    ShaderNodeId::eSID_PowFresnel(node)
}

pub fn draw_eSID_Power(mut node:eSID_Power, ui: &mut egui::Ui, _ctx: &egui::Context) -> ShaderNodeId {

    ShaderNodeId::eSID_Power(node)
}

pub fn draw_eSID_PulseWave(mut node:eSID_PulseWave, ui: &mut egui::Ui, _ctx: &egui::Context) -> ShaderNodeId {

    ShaderNodeId::eSID_PulseWave(node)
}

pub fn draw_eSID_RGB2UV(mut node:eSID_RGB2UV, ui: &mut egui::Ui, _ctx: &egui::Context) -> ShaderNodeId {

    ShaderNodeId::eSID_RGB2UV(node)
}

pub fn draw_eSID_RGB_Ramp(mut node:eSID_RGB_Ramp, ui: &mut egui::Ui, _ctx: &egui::Context) -> ShaderNodeId {

    ShaderNodeId::eSID_RGB_Ramp(node)
}

pub fn draw_eSID_RefracTex(mut node:eSID_RefracTex, ui: &mut egui::Ui, _ctx: &egui::Context) -> ShaderNodeId {

    ShaderNodeId::eSID_RefracTex(node)
}

pub fn draw_eSID_SUB(mut node:eSID_SUB, ui: &mut egui::Ui, _ctx: &egui::Context) -> ShaderNodeId {

    ShaderNodeId::eSID_SUB(node)
}

pub fn draw_eSID_SawWave(mut node:eSID_SawWave, ui: &mut egui::Ui, _ctx: &egui::Context) -> ShaderNodeId {

    ShaderNodeId::eSID_SawWave(node)
}

pub fn draw_eSID_ShadowValue(mut node:eSID_ShadowValue, ui: &mut egui::Ui, _ctx: &egui::Context) -> ShaderNodeId {

    ShaderNodeId::eSID_ShadowValue(node)
}

pub fn draw_eSID_SinusFX(mut node:eSID_SinusFX, ui: &mut egui::Ui, _ctx: &egui::Context) -> ShaderNodeId {

    ShaderNodeId::eSID_SinusFX(node)
}

pub fn draw_eSID_SpecularCubeMap(mut node:eSID_SpecularCubeMap, ui: &mut egui::Ui, _ctx: &egui::Context) -> ShaderNodeId {

    ShaderNodeId::eSID_SpecularCubeMap(node)
}

pub fn draw_eSID_SpecularGlossMultiplier(mut node:eSID_SpecularGlossMultiplier, ui: &mut egui::Ui, _ctx: &egui::Context) -> ShaderNodeId {

    ShaderNodeId::eSID_SpecularGlossMultiplier(node)
}

pub fn draw_eSID_SpecularPowerMultiplier(mut node:eSID_SpecularPowerMultiplier, ui: &mut egui::Ui, _ctx: &egui::Context) -> ShaderNodeId {

    ShaderNodeId::eSID_SpecularPowerMultiplier(node)
}

pub fn draw_eSID_Tangent2Screen(mut node:eSID_Tangent2Screen, ui: &mut egui::Ui, _ctx: &egui::Context) -> ShaderNodeId {

    ShaderNodeId::eSID_Tangent2Screen(node)
}

pub fn draw_eSID_Tex2D(mut node:eSID_Tex2D, ui: &mut egui::Ui, _ctx: &egui::Context) -> ShaderNodeId {

    ShaderNodeId::eSID_Tex2D(node)
}

pub fn draw_eSID_Tex2DYUV(mut node:eSID_Tex2DYUV, ui: &mut egui::Ui, _ctx: &egui::Context) -> ShaderNodeId {

    ShaderNodeId::eSID_Tex2DYUV(node)
}

pub fn draw_eSID_TexBump(mut node:eSID_TexBump, ui: &mut egui::Ui, _ctx: &egui::Context) -> ShaderNodeId {

    ShaderNodeId::eSID_TexBump(node)
}

pub fn draw_eSID_TexBumpTangent(mut node:eSID_TexBumpTangent, ui: &mut egui::Ui, _ctx: &egui::Context) -> ShaderNodeId {

    ShaderNodeId::eSID_TexBumpTangent(node)
}

pub fn draw_eSID_UV2RGB(mut node:eSID_UV2RGB, ui: &mut egui::Ui, _ctx: &egui::Context) -> ShaderNodeId {

    ShaderNodeId::eSID_UV2RGB(node)
}

pub fn draw_eSID_UVRotate(mut node:eSID_UVRotate, ui: &mut egui::Ui, _ctx: &egui::Context) -> ShaderNodeId {

    ShaderNodeId::eSID_UVRotate(node)
}

pub fn draw_eSID_UVScroll(mut node:eSID_UVScroll, ui: &mut egui::Ui, _ctx: &egui::Context) -> ShaderNodeId {

    ShaderNodeId::eSID_UVScroll(node)
}

pub fn draw_eSID_UVScrollSpeedFactor(mut node:eSID_UVScrollSpeedFactor, ui: &mut egui::Ui, _ctx: &egui::Context) -> ShaderNodeId {

    ShaderNodeId::eSID_UVScrollSpeedFactor(node)
}

pub fn draw_eSID_UVSource(mut node:eSID_UVSource, ui: &mut egui::Ui, _ctx: &egui::Context) -> ShaderNodeId {

    ShaderNodeId::eSID_UVSource(node)
}

pub fn draw_eSID_UV_ADD(mut node:eSID_UV_ADD, ui: &mut egui::Ui, _ctx: &egui::Context) -> ShaderNodeId {

    ShaderNodeId::eSID_UV_ADD(node)
}

pub fn draw_eSID_UV_Blender(mut node:eSID_UV_Blender, ui: &mut egui::Ui, _ctx: &egui::Context) -> ShaderNodeId {

    ShaderNodeId::eSID_UV_Blender(node)
}

pub fn draw_eSID_UV_Combiner4D(mut node:eSID_UV_Combiner4D, ui: &mut egui::Ui, _ctx: &egui::Context) -> ShaderNodeId {

    ShaderNodeId::eSID_UV_Combiner4D(node)
}

pub fn draw_eSID_UV_ConstantMUL(mut node:eSID_UV_ConstantMUL, ui: &mut egui::Ui, _ctx: &egui::Context) -> ShaderNodeId {

    ShaderNodeId::eSID_UV_ConstantMUL(node)
}

pub fn draw_eSID_UV_DUDV(mut node:eSID_UV_DUDV, ui: &mut egui::Ui, _ctx: &egui::Context) -> ShaderNodeId {

    ShaderNodeId::eSID_UV_DUDV(node)
}

pub fn draw_eSID_UV_MUL(mut node:eSID_UV_MUL, ui: &mut egui::Ui, _ctx: &egui::Context) -> ShaderNodeId {

    ShaderNodeId::eSID_UV_MUL(node)
}

pub fn draw_eSID_UV_SUB(mut node:eSID_UV_SUB, ui: &mut egui::Ui, _ctx: &egui::Context) -> ShaderNodeId {

    ShaderNodeId::eSID_UV_SUB(node)
}

pub fn draw_eSID_UntransformedNormal(mut node:eSID_UntransformedNormal, ui: &mut egui::Ui, _ctx: &egui::Context) -> ShaderNodeId {

    ShaderNodeId::eSID_UntransformedNormal(node)
}

pub fn draw_eSID_VERTEX_UV_SUB(mut node:eSID_VERTEX_UV_SUB, ui: &mut egui::Ui, _ctx: &egui::Context) -> ShaderNodeId {

    ShaderNodeId::eSID_VERTEX_UV_SUB(node)
}

pub fn draw_eSID_VertexColor(mut node:eSID_VertexColor, ui: &mut egui::Ui, _ctx: &egui::Context) -> ShaderNodeId {

    ShaderNodeId::eSID_VertexColor(node)
}

pub fn draw_eSID_VertexColorToPixel(mut node:eSID_VertexColorToPixel, ui: &mut egui::Ui, _ctx: &egui::Context) -> ShaderNodeId {

    ShaderNodeId::eSID_VertexColorToPixel(node)
}

pub fn draw_eSID_VertexColor_Misc1(mut node:eSID_VertexColor_Misc1, ui: &mut egui::Ui, _ctx: &egui::Context) -> ShaderNodeId {

    ShaderNodeId::eSID_VertexColor_Misc1(node)
}

pub fn draw_eSID_VertexColor_Misc2(mut node:eSID_VertexColor_Misc2, ui: &mut egui::Ui, _ctx: &egui::Context) -> ShaderNodeId {

    ShaderNodeId::eSID_VertexColor_Misc2(node)
}

pub fn draw_eSID_VertexCombiner4D(mut node:eSID_VertexCombiner4D, ui: &mut egui::Ui, _ctx: &egui::Context) -> ShaderNodeId {

    ShaderNodeId::eSID_VertexCombiner4D(node)
}

pub fn draw_eSID_VertexConstUVWQ(mut node:eSID_VertexConstUVWQ, ui: &mut egui::Ui, _ctx: &egui::Context) -> ShaderNodeId {

    ShaderNodeId::eSID_VertexConstUVWQ(node)
}

pub fn draw_eSID_VertexConstantVector(mut node:eSID_VertexConstantVector, ui: &mut egui::Ui, _ctx: &egui::Context) -> ShaderNodeId {

    ShaderNodeId::eSID_VertexConstantVector(node)
}

pub fn draw_eSID_VertexElapseTime(mut node:eSID_VertexElapseTime, ui: &mut egui::Ui, _ctx: &egui::Context) -> ShaderNodeId {

    ShaderNodeId::eSID_VertexElapseTime(node)
}

pub fn draw_eSID_VertexInvert(mut node:eSID_VertexInvert, ui: &mut egui::Ui, _ctx: &egui::Context) -> ShaderNodeId {

    ShaderNodeId::eSID_VertexInvert(node)
}

pub fn draw_eSID_VertexNormal(mut node:eSID_VertexNormal, ui: &mut egui::Ui, _ctx: &egui::Context) -> ShaderNodeId {

    ShaderNodeId::eSID_VertexNormal(node)
}

pub fn draw_eSID_VertexPulseWave(mut node:eSID_VertexPulseWave, ui: &mut egui::Ui, _ctx: &egui::Context) -> ShaderNodeId {

    ShaderNodeId::eSID_VertexPulseWave(node)
}

pub fn draw_eSID_VertexRGB2UV(mut node:eSID_VertexRGB2UV, ui: &mut egui::Ui, _ctx: &egui::Context) -> ShaderNodeId {

    ShaderNodeId::eSID_VertexRGB2UV(node)
}

pub fn draw_eSID_VertexUVRotate(mut node:eSID_VertexUVRotate, ui: &mut egui::Ui, _ctx: &egui::Context) -> ShaderNodeId {

    ShaderNodeId::eSID_VertexUVRotate(node)
}

pub fn draw_eSID_VertexUVScroll(mut node:eSID_VertexUVScroll, ui: &mut egui::Ui, _ctx: &egui::Context) -> ShaderNodeId {

    ShaderNodeId::eSID_VertexUVScroll(node)
}

pub fn draw_eSID_VertexUVSource(mut node:eSID_VertexUVSource, ui: &mut egui::Ui, _ctx: &egui::Context) -> ShaderNodeId {

    ShaderNodeId::eSID_VertexUVSource(node)
}

pub fn draw_eSID_VertexUVToPixelUV(mut node:eSID_VertexUVToPixelUV, ui: &mut egui::Ui, _ctx: &egui::Context) -> ShaderNodeId {

    ShaderNodeId::eSID_VertexUVToPixelUV(node)
}

pub fn draw_eSID_VertexUV_Combiner4D(mut node:eSID_VertexUV_Combiner4D, ui: &mut egui::Ui, _ctx: &egui::Context) -> ShaderNodeId {

    ShaderNodeId::eSID_VertexUV_Combiner4D(node)
}

pub fn draw_eSID_VertexUntransformedNormal(mut node:eSID_VertexUntransformedNormal, ui: &mut egui::Ui, _ctx: &egui::Context) -> ShaderNodeId {

    ShaderNodeId::eSID_VertexUntransformedNormal(node)
}

pub fn draw_eSID_Vertex_UV_ADD(mut node:eSID_Vertex_UV_ADD, ui: &mut egui::Ui, _ctx: &egui::Context) -> ShaderNodeId {

    ShaderNodeId::eSID_Vertex_UV_ADD(node)
}

pub fn draw_eSID_Vertex_UV_Blender(mut node:eSID_Vertex_UV_Blender, ui: &mut egui::Ui, _ctx: &egui::Context) -> ShaderNodeId {

    ShaderNodeId::eSID_Vertex_UV_Blender(node)
}

pub fn draw_eSID_Vertex_UV_ConstantMUL(mut node:eSID_Vertex_UV_ConstantMUL, ui: &mut egui::Ui, _ctx: &egui::Context) -> ShaderNodeId {

    ShaderNodeId::eSID_Vertex_UV_ConstantMUL(node)
}

pub fn draw_eSID_Vertex_UV_MUL(mut node:eSID_Vertex_UV_MUL, ui: &mut egui::Ui, _ctx: &egui::Context) -> ShaderNodeId {

    ShaderNodeId::eSID_Vertex_UV_MUL(node)
}

pub fn draw_eSID_ViewPosition(mut node:eSID_ViewPosition, ui: &mut egui::Ui, _ctx: &egui::Context) -> ShaderNodeId {

    ShaderNodeId::eSID_ViewPosition(node)
}

pub fn draw_eSID_VolumeAlphaEx(mut node:eSID_VolumeAlphaEx, ui: &mut egui::Ui, _ctx: &egui::Context) -> ShaderNodeId {

    ShaderNodeId::eSID_VolumeAlphaEx(node)
}

pub fn draw_eSID_VolumeAmbLightIntensityOpt(mut node:eSID_VolumeAmbLightIntensityOpt, ui: &mut egui::Ui, _ctx: &egui::Context) -> ShaderNodeId {

    ShaderNodeId::eSID_VolumeAmbLightIntensityOpt(node)
}

pub fn draw_eSID_VolumetricParticle(mut node:eSID_VolumetricParticle, ui: &mut egui::Ui, _ctx: &egui::Context) -> ShaderNodeId {

    ShaderNodeId::eSID_VolumetricParticle(node)
}

pub fn draw_eSID_VtxWorldPosition(mut node:eSID_VtxWorldPosition, ui: &mut egui::Ui, _ctx: &egui::Context) -> ShaderNodeId {

    ShaderNodeId::eSID_VtxWorldPosition(node)
}

pub fn draw_eSID_WorldPosition(mut node:eSID_WorldPosition, ui: &mut egui::Ui, _ctx: &egui::Context) -> ShaderNodeId {

    ShaderNodeId::eSID_WorldPosition(node)
}

pub fn draw_eSID_WorldSinusFX_VC(mut node:eSID_WorldSinusFX_VC, ui: &mut egui::Ui, _ctx: &egui::Context) -> ShaderNodeId {

    ShaderNodeId::eSID_WorldSinusFX_VC(node)
}