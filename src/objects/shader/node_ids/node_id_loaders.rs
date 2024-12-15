#![allow(non_snake_case, dead_code, unused_variables)]

use crate::objects::YetiIOError;
use super::*;
use std::io::{Read, Seek, SeekFrom};
use byteorder::{ReadBytesExt, LittleEndian};

pub fn load_eSID_ADD<T: Read + Seek>(rdr: &mut T) -> Result<eSID_ADD, YetiIOError> {

    Ok(eSID_ADD {
        
    })
}
pub fn load_eSID_AmbientCube<T: Read + Seek>(rdr: &mut T) -> Result<eSID_AmbientCube, YetiIOError> {

    Ok(eSID_AmbientCube {
        
    })
}
pub fn load_eSID_AmbientOcclusion<T: Read + Seek>(rdr: &mut T) -> Result<eSID_AmbientOcclusion, YetiIOError> {

    Ok(eSID_AmbientOcclusion {
        
    })
}
pub fn load_eSID_Blender<T: Read + Seek>(rdr: &mut T) -> Result<eSID_Blender, YetiIOError> {

    Ok(eSID_Blender {
        
    })
}
pub fn load_eSID_BlurTex<T: Read + Seek>(rdr: &mut T) -> Result<eSID_BlurTex, YetiIOError> {

    Ok(eSID_BlurTex {
        
    })
}
pub fn load_eSID_Clamp<T: Read + Seek>(rdr: &mut T) -> Result<eSID_Clamp, YetiIOError> {

    Ok(eSID_Clamp {
        
    })
}
pub fn load_eSID_ColorSelector<T: Read + Seek>(rdr: &mut T) -> Result<eSID_ColorSelector, YetiIOError> {

    Ok(eSID_ColorSelector {
        
    })
}
pub fn load_eSID_ColorSelectorSmooth<T: Read + Seek>(rdr: &mut T) -> Result<eSID_ColorSelectorSmooth, YetiIOError> {

    Ok(eSID_ColorSelectorSmooth {
        
    })
}
pub fn load_eSID_Combiner4D<T: Read + Seek>(rdr: &mut T) -> Result<eSID_Combiner4D, YetiIOError> {

    Ok(eSID_Combiner4D {
        
    })
}
pub fn load_eSID_Comment<T: Read + Seek>(rdr: &mut T) -> Result<eSID_Comment, YetiIOError> {

    rdr.read_u32::<LittleEndian>()?;

    let len = rdr.read_u32::<LittleEndian>()?;
    //dbg!(&len);
    let mut v: Vec<u8> = vec![0; len as usize];
    for b in v.iter_mut() {
        *b = rdr.read_u8()?;
    }
    //dbg!(&v);
    
    let rem = len % 4;
    rdr.seek(SeekFrom::Current(4 - rem as i64))?;

    let comment = String::from_utf8(v)?;
    //info!("comment: {}", &comment);

    Ok(eSID_Comment {
        comment,
    })
}
pub fn load_eSID_ConstantColor<T: Read + Seek>(rdr: &mut T) -> Result<eSID_ConstantColor, YetiIOError> {

    Ok(eSID_ConstantColor {
        
    })
}
pub fn load_eSID_ConstantMUL<T: Read + Seek>(rdr: &mut T) -> Result<eSID_ConstantMUL, YetiIOError> {

    Ok(eSID_ConstantMUL {
        
    })
}
pub fn load_eSID_ConstantUVWQ<T: Read + Seek>(rdr: &mut T) -> Result<eSID_ConstantUVWQ, YetiIOError> {

    Ok(eSID_ConstantUVWQ {
        
    })
}
pub fn load_eSID_ConstantVector<T: Read + Seek>(rdr: &mut T) -> Result<eSID_ConstantVector, YetiIOError> {

    Ok(eSID_ConstantVector {
        
    })
}
pub fn load_eSID_CustomCode<T: Read + Seek>(rdr: &mut T) -> Result<eSID_CustomCode, YetiIOError> {

    Ok(eSID_CustomCode {
        
    })
}
pub fn load_eSID_CustomCodeVtx<T: Read + Seek>(rdr: &mut T) -> Result<eSID_CustomCodeVtx, YetiIOError> {

    Ok(eSID_CustomCodeVtx {
        
    })
}
pub fn load_eSID_DepthAlpha<T: Read + Seek>(rdr: &mut T) -> Result<eSID_DepthAlpha, YetiIOError> {

    Ok(eSID_DepthAlpha {
        
    })
}
pub fn load_eSID_DepthAlphaOpt<T: Read + Seek>(rdr: &mut T) -> Result<eSID_DepthAlphaOpt, YetiIOError> {

    Ok(eSID_DepthAlphaOpt {
        
    })
}
pub fn load_eSID_DepthTexture<T: Read + Seek>(rdr: &mut T) -> Result<eSID_DepthTexture, YetiIOError> {

    Ok(eSID_DepthTexture {
        
    })
}
pub fn load_eSID_DiffuseCube<T: Read + Seek>(rdr: &mut T) -> Result<eSID_DiffuseCube, YetiIOError> {

    Ok(eSID_DiffuseCube {
        
    })
}
pub fn load_eSID_DiffuseMultiplier<T: Read + Seek>(rdr: &mut T) -> Result<eSID_DiffuseMultiplier, YetiIOError> {

    Ok(eSID_DiffuseMultiplier {
        
    })
}
pub fn load_eSID_DustFXSH<T: Read + Seek>(rdr: &mut T) -> Result<eSID_DustFXSH, YetiIOError> {

    Ok(eSID_DustFXSH {
        
    })
}
pub fn load_eSID_ElapseTime<T: Read + Seek>(rdr: &mut T) -> Result<eSID_ElapseTime, YetiIOError> {

    Ok(eSID_ElapseTime {
        
    })
}
pub fn load_eSID_FlatChrome<T: Read + Seek>(rdr: &mut T) -> Result<eSID_FlatChrome, YetiIOError> {

    Ok(eSID_FlatChrome {
        
    })
}
pub fn load_eSID_ForceNoPrepass<T: Read + Seek>(rdr: &mut T) -> Result<eSID_ForceNoPrepass, YetiIOError> {

    Ok(eSID_ForceNoPrepass {
        
    })
}
pub fn load_eSID_ForceUltraSimpleShader<T: Read + Seek>(rdr: &mut T) -> Result<eSID_ForceUltraSimpleShader, YetiIOError> {

    Ok(eSID_ForceUltraSimpleShader {
        
    })
}
pub fn load_eSID_ForceVisualPrepass<T: Read + Seek>(rdr: &mut T) -> Result<eSID_ForceVisualPrepass, YetiIOError> {

    Ok(eSID_ForceVisualPrepass {
        
    })
}
pub fn load_eSID_HeatSelector<T: Read + Seek>(rdr: &mut T) -> Result<eSID_HeatSelector, YetiIOError> {

    Ok(eSID_HeatSelector {
        
    })
}
pub fn load_eSID_Invert<T: Read + Seek>(rdr: &mut T) -> Result<eSID_Invert, YetiIOError> {

    Ok(eSID_Invert {
        
    })
}
pub fn load_eSID_LODBlender<T: Read + Seek>(rdr: &mut T) -> Result<eSID_LODBlender, YetiIOError> {

    Ok(eSID_LODBlender {
        
    })
}
pub fn load_eSID_LODMUL<T: Read + Seek>(rdr: &mut T) -> Result<eSID_LODMUL, YetiIOError> {

    Ok(eSID_LODMUL {
        
    })
}
pub fn load_eSID_LightCurve_Back<T: Read + Seek>(rdr: &mut T) -> Result<eSID_LightCurve_Back, YetiIOError> {

    Ok(eSID_LightCurve_Back {
        
    })
}
pub fn load_eSID_LinearFresnel<T: Read + Seek>(rdr: &mut T) -> Result<eSID_LinearFresnel, YetiIOError> {

    Ok(eSID_LinearFresnel {
        
    })
}
pub fn load_eSID_Luminance<T: Read + Seek>(rdr: &mut T) -> Result<eSID_Luminance, YetiIOError> {

    Ok(eSID_Luminance {
        
    })
}
pub fn load_eSID_MUL<T: Read + Seek>(rdr: &mut T) -> Result<eSID_MUL, YetiIOError> {

    Ok(eSID_MUL {
        
    })
}
pub fn load_eSID_MUL2X<T: Read + Seek>(rdr: &mut T) -> Result<eSID_MUL2X, YetiIOError> {

    Ok(eSID_MUL2X {
        
    })
}
pub fn load_eSID_MainOutput<T: Read + Seek>(rdr: &mut T) -> Result<eSID_MainOutput, YetiIOError> {

    Ok(eSID_MainOutput {
        
    })
}
pub fn load_eSID_MaterialColor_Diffuse<T: Read + Seek>(rdr: &mut T) -> Result<eSID_MaterialColor_Diffuse, YetiIOError> {

    Ok(eSID_MaterialColor_Diffuse {
        
    })
}
pub fn load_eSID_MaterialColor_Emissive<T: Read + Seek>(rdr: &mut T) -> Result<eSID_MaterialColor_Emissive, YetiIOError> {

    Ok(eSID_MaterialColor_Emissive {
        
    })
}
pub fn load_eSID_MaterialColor_EmissiveVTX<T: Read + Seek>(rdr: &mut T) -> Result<eSID_MaterialColor_EmissiveVTX, YetiIOError> {

    Ok(eSID_MaterialColor_EmissiveVTX {
        
    })
}
pub fn load_eSID_MaterialColor_Specular<T: Read + Seek>(rdr: &mut T) -> Result<eSID_MaterialColor_Specular, YetiIOError> {

    Ok(eSID_MaterialColor_Specular {
        
    })
}
pub fn load_eSID_Normal<T: Read + Seek>(rdr: &mut T) -> Result<eSID_Normal, YetiIOError> {

    Ok(eSID_Normal {
        
    })
}
pub fn load_eSID_Normalize3D<T: Read + Seek>(rdr: &mut T) -> Result<eSID_Normalize3D, YetiIOError> {

    Ok(eSID_Normalize3D {
        
    })
}
pub fn load_eSID_OffsetBump<T: Read + Seek>(rdr: &mut T) -> Result<eSID_OffsetBump, YetiIOError> {

    Ok(eSID_OffsetBump {
        
    })
}
pub fn load_eSID_PixelAverageColor<T: Read + Seek>(rdr: &mut T) -> Result<eSID_PixelAverageColor, YetiIOError> {

    Ok(eSID_PixelAverageColor {
        
    })
}
pub fn load_eSID_PixelColor_Misc<T: Read + Seek>(rdr: &mut T) -> Result<eSID_PixelColor_Misc, YetiIOError> {

    Ok(eSID_PixelColor_Misc {
        
    })
}
pub fn load_eSID_PixelColor_Misc2<T: Read + Seek>(rdr: &mut T) -> Result<eSID_PixelColor_Misc2, YetiIOError> {

    Ok(eSID_PixelColor_Misc2 {
        
    })
}
pub fn load_eSID_PixelSH<T: Read + Seek>(rdr: &mut T) -> Result<eSID_PixelSH, YetiIOError> {

    Ok(eSID_PixelSH {
        
    })
}
pub fn load_eSID_PixelUVBoxAnimBlend<T: Read + Seek>(rdr: &mut T) -> Result<eSID_PixelUVBoxAnimBlend, YetiIOError> {

    Ok(eSID_PixelUVBoxAnimBlend {
        
    })
}
pub fn load_eSID_PixelViewToWorld<T: Read + Seek>(rdr: &mut T) -> Result<eSID_PixelViewToWorld, YetiIOError> {

    Ok(eSID_PixelViewToWorld {
        
    })
}
pub fn load_eSID_Position<T: Read + Seek>(rdr: &mut T) -> Result<eSID_Position, YetiIOError> {

    Ok(eSID_Position {
        
    })
}
pub fn load_eSID_PowFresnel<T: Read + Seek>(rdr: &mut T) -> Result<eSID_PowFresnel, YetiIOError> {

    Ok(eSID_PowFresnel {
        
    })
}
pub fn load_eSID_Power<T: Read + Seek>(rdr: &mut T) -> Result<eSID_Power, YetiIOError> {

    Ok(eSID_Power {
        
    })
}
pub fn load_eSID_PulseWave<T: Read + Seek>(rdr: &mut T) -> Result<eSID_PulseWave, YetiIOError> {

    Ok(eSID_PulseWave {
        
    })
}
pub fn load_eSID_RGB2UV<T: Read + Seek>(rdr: &mut T) -> Result<eSID_RGB2UV, YetiIOError> {

    Ok(eSID_RGB2UV {
        
    })
}
pub fn load_eSID_RGB_Ramp<T: Read + Seek>(rdr: &mut T) -> Result<eSID_RGB_Ramp, YetiIOError> {

    Ok(eSID_RGB_Ramp {
        
    })
}
pub fn load_eSID_RefracTex<T: Read + Seek>(rdr: &mut T) -> Result<eSID_RefracTex, YetiIOError> {

    Ok(eSID_RefracTex {
        
    })
}
pub fn load_eSID_SUB<T: Read + Seek>(rdr: &mut T) -> Result<eSID_SUB, YetiIOError> {

    Ok(eSID_SUB {
        
    })
}
pub fn load_eSID_SawWave<T: Read + Seek>(rdr: &mut T) -> Result<eSID_SawWave, YetiIOError> {

    Ok(eSID_SawWave {
        
    })
}
pub fn load_eSID_ShadowValue<T: Read + Seek>(rdr: &mut T) -> Result<eSID_ShadowValue, YetiIOError> {

    Ok(eSID_ShadowValue {
        
    })
}
pub fn load_eSID_SinusFX<T: Read + Seek>(rdr: &mut T) -> Result<eSID_SinusFX, YetiIOError> {

    Ok(eSID_SinusFX {
        
    })
}
pub fn load_eSID_SpecularCubeMap<T: Read + Seek>(rdr: &mut T) -> Result<eSID_SpecularCubeMap, YetiIOError> {

    Ok(eSID_SpecularCubeMap {
        
    })
}
pub fn load_eSID_SpecularGlossMultiplier<T: Read + Seek>(rdr: &mut T) -> Result<eSID_SpecularGlossMultiplier, YetiIOError> {

    Ok(eSID_SpecularGlossMultiplier {
        
    })
}
pub fn load_eSID_SpecularPowerMultiplier<T: Read + Seek>(rdr: &mut T) -> Result<eSID_SpecularPowerMultiplier, YetiIOError> {

    Ok(eSID_SpecularPowerMultiplier {
        
    })
}
pub fn load_eSID_Tangent2Screen<T: Read + Seek>(rdr: &mut T) -> Result<eSID_Tangent2Screen, YetiIOError> {

    Ok(eSID_Tangent2Screen {
        
    })
}
pub fn load_eSID_Tex2D<T: Read + Seek>(rdr: &mut T) -> Result<eSID_Tex2D, YetiIOError> {

    Ok(eSID_Tex2D {
        
    })
}
pub fn load_eSID_Tex2DYUV<T: Read + Seek>(rdr: &mut T) -> Result<eSID_Tex2DYUV, YetiIOError> {

    Ok(eSID_Tex2DYUV {
        
    })
}
pub fn load_eSID_TexBump<T: Read + Seek>(rdr: &mut T) -> Result<eSID_TexBump, YetiIOError> {

    Ok(eSID_TexBump {
        
    })
}
pub fn load_eSID_TexBumpTangent<T: Read + Seek>(rdr: &mut T) -> Result<eSID_TexBumpTangent, YetiIOError> {

    Ok(eSID_TexBumpTangent {
        
    })
}
pub fn load_eSID_UV2RGB<T: Read + Seek>(rdr: &mut T) -> Result<eSID_UV2RGB, YetiIOError> {

    Ok(eSID_UV2RGB {
        
    })
}
pub fn load_eSID_UVRotate<T: Read + Seek>(rdr: &mut T) -> Result<eSID_UVRotate, YetiIOError> {

    Ok(eSID_UVRotate {
        
    })
}
pub fn load_eSID_UVScroll<T: Read + Seek>(rdr: &mut T) -> Result<eSID_UVScroll, YetiIOError> {

    Ok(eSID_UVScroll {
        
    })
}
pub fn load_eSID_UVScrollSpeedFactor<T: Read + Seek>(rdr: &mut T) -> Result<eSID_UVScrollSpeedFactor, YetiIOError> {

    Ok(eSID_UVScrollSpeedFactor {
        
    })
}
pub fn load_eSID_UVSource<T: Read + Seek>(rdr: &mut T) -> Result<eSID_UVSource, YetiIOError> {

    Ok(eSID_UVSource {
        
    })
}
pub fn load_eSID_UV_ADD<T: Read + Seek>(rdr: &mut T) -> Result<eSID_UV_ADD, YetiIOError> {

    Ok(eSID_UV_ADD {
        
    })
}
pub fn load_eSID_UV_Blender<T: Read + Seek>(rdr: &mut T) -> Result<eSID_UV_Blender, YetiIOError> {

    Ok(eSID_UV_Blender {
        
    })
}
pub fn load_eSID_UV_Combiner4D<T: Read + Seek>(rdr: &mut T) -> Result<eSID_UV_Combiner4D, YetiIOError> {

    Ok(eSID_UV_Combiner4D {
        
    })
}
pub fn load_eSID_UV_ConstantMUL<T: Read + Seek>(rdr: &mut T) -> Result<eSID_UV_ConstantMUL, YetiIOError> {

    Ok(eSID_UV_ConstantMUL {
        
    })
}
pub fn load_eSID_UV_DUDV<T: Read + Seek>(rdr: &mut T) -> Result<eSID_UV_DUDV, YetiIOError> {

    Ok(eSID_UV_DUDV {
        
    })
}
pub fn load_eSID_UV_MUL<T: Read + Seek>(rdr: &mut T) -> Result<eSID_UV_MUL, YetiIOError> {

    Ok(eSID_UV_MUL {
        
    })
}
pub fn load_eSID_UV_SUB<T: Read + Seek>(rdr: &mut T) -> Result<eSID_UV_SUB, YetiIOError> {

    Ok(eSID_UV_SUB {
        
    })
}
pub fn load_eSID_UntransformedNormal<T: Read + Seek>(rdr: &mut T) -> Result<eSID_UntransformedNormal, YetiIOError> {

    Ok(eSID_UntransformedNormal {
        
    })
}
pub fn load_eSID_VERTEX_UV_SUB<T: Read + Seek>(rdr: &mut T) -> Result<eSID_VERTEX_UV_SUB, YetiIOError> {

    Ok(eSID_VERTEX_UV_SUB {
        
    })
}
pub fn load_eSID_VertexColor<T: Read + Seek>(rdr: &mut T) -> Result<eSID_VertexColor, YetiIOError> {

    Ok(eSID_VertexColor {
        
    })
}
pub fn load_eSID_VertexColorToPixel<T: Read + Seek>(rdr: &mut T) -> Result<eSID_VertexColorToPixel, YetiIOError> {

    Ok(eSID_VertexColorToPixel {
        
    })
}
pub fn load_eSID_VertexColor_Misc1<T: Read + Seek>(rdr: &mut T) -> Result<eSID_VertexColor_Misc1, YetiIOError> {

    Ok(eSID_VertexColor_Misc1 {
        
    })
}
pub fn load_eSID_VertexColor_Misc2<T: Read + Seek>(rdr: &mut T) -> Result<eSID_VertexColor_Misc2, YetiIOError> {

    Ok(eSID_VertexColor_Misc2 {
        
    })
}
pub fn load_eSID_VertexCombiner4D<T: Read + Seek>(rdr: &mut T) -> Result<eSID_VertexCombiner4D, YetiIOError> {

    Ok(eSID_VertexCombiner4D {
        
    })
}
pub fn load_eSID_VertexConstUVWQ<T: Read + Seek>(rdr: &mut T) -> Result<eSID_VertexConstUVWQ, YetiIOError> {

    Ok(eSID_VertexConstUVWQ {
        
    })
}
pub fn load_eSID_VertexConstantVector<T: Read + Seek>(rdr: &mut T) -> Result<eSID_VertexConstantVector, YetiIOError> {

    Ok(eSID_VertexConstantVector {
        
    })
}
pub fn load_eSID_VertexElapseTime<T: Read + Seek>(rdr: &mut T) -> Result<eSID_VertexElapseTime, YetiIOError> {

    Ok(eSID_VertexElapseTime {
        
    })
}
pub fn load_eSID_VertexInvert<T: Read + Seek>(rdr: &mut T) -> Result<eSID_VertexInvert, YetiIOError> {

    Ok(eSID_VertexInvert {
        
    })
}
pub fn load_eSID_VertexNormal<T: Read + Seek>(rdr: &mut T) -> Result<eSID_VertexNormal, YetiIOError> {

    Ok(eSID_VertexNormal {
        
    })
}
pub fn load_eSID_VertexPulseWave<T: Read + Seek>(rdr: &mut T) -> Result<eSID_VertexPulseWave, YetiIOError> {

    Ok(eSID_VertexPulseWave {
        
    })
}
pub fn load_eSID_VertexRGB2UV<T: Read + Seek>(rdr: &mut T) -> Result<eSID_VertexRGB2UV, YetiIOError> {

    Ok(eSID_VertexRGB2UV {
        
    })
}
pub fn load_eSID_VertexUVRotate<T: Read + Seek>(rdr: &mut T) -> Result<eSID_VertexUVRotate, YetiIOError> {

    Ok(eSID_VertexUVRotate {
        
    })
}
pub fn load_eSID_VertexUVScroll<T: Read + Seek>(rdr: &mut T) -> Result<eSID_VertexUVScroll, YetiIOError> {

    Ok(eSID_VertexUVScroll {
        
    })
}
pub fn load_eSID_VertexUVSource<T: Read + Seek>(rdr: &mut T) -> Result<eSID_VertexUVSource, YetiIOError> {

    Ok(eSID_VertexUVSource {
        
    })
}
pub fn load_eSID_VertexUVToPixelUV<T: Read + Seek>(rdr: &mut T) -> Result<eSID_VertexUVToPixelUV, YetiIOError> {

    Ok(eSID_VertexUVToPixelUV {
        
    })
}
pub fn load_eSID_VertexUV_Combiner4D<T: Read + Seek>(rdr: &mut T) -> Result<eSID_VertexUV_Combiner4D, YetiIOError> {

    Ok(eSID_VertexUV_Combiner4D {
        
    })
}
pub fn load_eSID_VertexUntransformedNormal<T: Read + Seek>(rdr: &mut T) -> Result<eSID_VertexUntransformedNormal, YetiIOError> {

    Ok(eSID_VertexUntransformedNormal {
        
    })
}
pub fn load_eSID_Vertex_UV_ADD<T: Read + Seek>(rdr: &mut T) -> Result<eSID_Vertex_UV_ADD, YetiIOError> {

    Ok(eSID_Vertex_UV_ADD {
        
    })
}
pub fn load_eSID_Vertex_UV_Blender<T: Read + Seek>(rdr: &mut T) -> Result<eSID_Vertex_UV_Blender, YetiIOError> {

    Ok(eSID_Vertex_UV_Blender {
        
    })
}
pub fn load_eSID_Vertex_UV_ConstantMUL<T: Read + Seek>(rdr: &mut T) -> Result<eSID_Vertex_UV_ConstantMUL, YetiIOError> {

    Ok(eSID_Vertex_UV_ConstantMUL {
        
    })
}
pub fn load_eSID_Vertex_UV_MUL<T: Read + Seek>(rdr: &mut T) -> Result<eSID_Vertex_UV_MUL, YetiIOError> {

    Ok(eSID_Vertex_UV_MUL {
        
    })
}
pub fn load_eSID_ViewPosition<T: Read + Seek>(rdr: &mut T) -> Result<eSID_ViewPosition, YetiIOError> {

    Ok(eSID_ViewPosition {
        
    })
}
pub fn load_eSID_VolumeAlphaEx<T: Read + Seek>(rdr: &mut T) -> Result<eSID_VolumeAlphaEx, YetiIOError> {

    Ok(eSID_VolumeAlphaEx {
        
    })
}
pub fn load_eSID_VolumeAmbLightIntensityOpt<T: Read + Seek>(rdr: &mut T) -> Result<eSID_VolumeAmbLightIntensityOpt, YetiIOError> {

    Ok(eSID_VolumeAmbLightIntensityOpt {
        
    })
}
pub fn load_eSID_VolumetricParticle<T: Read + Seek>(rdr: &mut T) -> Result<eSID_VolumetricParticle, YetiIOError> {

    Ok(eSID_VolumetricParticle {
        
    })
}
pub fn load_eSID_VtxWorldPosition<T: Read + Seek>(rdr: &mut T) -> Result<eSID_VtxWorldPosition, YetiIOError> {

    Ok(eSID_VtxWorldPosition {
        
    })
}
pub fn load_eSID_WorldPosition<T: Read + Seek>(rdr: &mut T) -> Result<eSID_WorldPosition, YetiIOError> {

    Ok(eSID_WorldPosition {
        
    })
}
pub fn load_eSID_WorldSinusFX<T: Read + Seek>(rdr: &mut T) -> Result<eSID_WorldSinusFX_VC, YetiIOError> {

    Ok(eSID_WorldSinusFX_VC {
        
    })
}