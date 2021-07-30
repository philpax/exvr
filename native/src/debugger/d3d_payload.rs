use crate::debugger::payload::*;

use bindings::Windows::Win32::Foundation::{BOOL, RECT};
use bindings::Windows::Win32::Graphics::Direct3D11::{
    D3D11_BOX, D3D11_MAP, D3D11_MAPPED_SUBRESOURCE, D3D11_VIEWPORT, D3D_PRIMITIVE_TOPOLOGY,
};
use bindings::Windows::Win32::Graphics::Dxgi::DXGI_FORMAT;
use windows::{Guid, IUnknown};

use std::os::raw::c_void;

use strum::EnumCount;
use strum_macros::{Display, EnumCount, EnumDiscriminants};

use cimgui as ig;

#[derive(Display, EnumDiscriminants, EnumCount, Clone)]
#[allow(dead_code)]
#[rustfmt::skip]
pub enum D3DPayload {
    QueryInterface(*const Guid, *mut *mut c_void),
    AddRef(),
    Release(),
    GetDevice(*mut *mut c_void),
    GetPrivateData(*const Guid, *mut u32, *mut c_void),
    SetPrivateData(*const Guid, u32, *mut c_void),
    SetPrivateDataInterface(*const Guid, *mut IUnknown),
    VSSetConstantBuffers(u32, u32, *mut *const c_void),
    PSSetShaderResources(u32, u32, *mut *const c_void),
    PSSetShader(*mut c_void, *mut *const c_void, u32),
    PSSetSamplers(u32, u32, *mut *const c_void),
    VSSetShader(*mut c_void, *mut *const c_void, u32),
    DrawIndexed(u32, u32, i32),
    Draw(u32, u32),
    Map(*mut c_void, u32, D3D11_MAP, u32, *mut D3D11_MAPPED_SUBRESOURCE),
    Unmap(*mut c_void, u32),
    PSSetConstantBuffers(u32, u32, *mut *const c_void),
    IASetInputLayout(*mut c_void),
    IASetVertexBuffers(u32, u32, *mut *const c_void, *mut u32, *mut u32),
    IASetIndexBuffer(*mut c_void, DXGI_FORMAT, u32),
    DrawIndexedInstanced(u32, u32, u32, i32, u32),
    DrawInstanced(u32, u32, u32, u32),
    GSSetConstantBuffers(u32, u32, *mut *const c_void),
    GSSetShader(*mut c_void, *mut *const c_void, u32),
    IASetPrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY),
    VSSetShaderResources(u32, u32, *mut *const c_void),
    VSSetSamplers(u32, u32, *mut *const c_void),
    Begin(*mut c_void),
    End(*mut c_void),
    GetData(*mut c_void, *mut c_void, u32, u32),
    SetPredication(*mut c_void, BOOL),
    GSSetShaderResources(u32, u32, *mut *const c_void),
    GSSetSamplers(u32, u32, *mut *const c_void),
    OMSetRenderTargets(u32, *mut *const c_void, *mut c_void),
    OMSetRenderTargetsAndUnorderedAccessViews(u32, *mut *const c_void, *mut c_void, u32, u32, *mut *const c_void, *mut u32),
    OMSetBlendState(*mut c_void, *mut f32, u32),
    OMSetDepthStencilState(*mut c_void, u32),
    SOSetTargets(u32, *mut *const c_void, *mut u32),
    DrawAuto(),
    DrawIndexedInstancedIndirect(*mut c_void, u32),
    DrawInstancedIndirect(*mut c_void, u32),
    Dispatch(u32, u32, u32),
    DispatchIndirect(*mut c_void, u32),
    RSSetState(*mut c_void),
    RSSetViewports(u32, *mut D3D11_VIEWPORT),
    RSSetScissorRects(u32, *mut RECT),
    CopySubresourceRegion(*mut c_void, u32, u32, u32, u32, *mut c_void, u32, *mut D3D11_BOX),
    CopyResource(*mut c_void, *mut c_void),
    UpdateSubresource(*mut c_void, u32, *mut D3D11_BOX, *mut c_void, u32, u32),
    CopyStructureCount(*mut c_void, u32, *mut c_void),
    ClearRenderTargetView(*mut c_void, *mut f32),
    ClearUnorderedAccessViewUint(*mut c_void, *mut u32),
    ClearUnorderedAccessViewFloat(*mut c_void, *mut f32),
    ClearDepthStencilView(*mut c_void, u32, f32, u8),
    GenerateMips(*mut c_void),
    SetResourceMinLOD(*mut c_void, f32),
    GetResourceMinLOD(*mut c_void),
    ResolveSubresource(*mut c_void, u32, *mut c_void, u32, DXGI_FORMAT),
    ExecuteCommandList(*mut c_void, BOOL),
    HSSetShaderResources(u32, u32, *mut *const c_void),
    HSSetShader(*mut c_void, *mut *const c_void, u32),
    HSSetSamplers(u32, u32, *mut *const c_void),
    HSSetConstantBuffers(u32, u32, *mut *const c_void),
    DSSetShaderResources(u32, u32, *mut *const c_void),
    DSSetShader(*mut c_void, *mut *const c_void, u32),
    DSSetSamplers(u32, u32, *mut *const c_void),
    DSSetConstantBuffers(u32, u32, *mut *const c_void),
    CSSetShaderResources(u32, u32, *mut *const c_void),
    CSSetUnorderedAccessViews(u32, u32, *mut *const c_void, *mut u32),
    CSSetShader(*mut c_void, *mut *const c_void, u32),
    CSSetSamplers(u32, u32, *mut *const c_void),
    CSSetConstantBuffers(u32, u32, *mut *const c_void),
    VSGetConstantBuffers(u32, u32, *mut *mut c_void),
    PSGetShaderResources(u32, u32, *mut *mut c_void),
    PSGetShader(*mut *mut c_void, *mut *mut c_void, *mut u32),
    PSGetSamplers(u32, u32, *mut *mut c_void),
    VSGetShader(*mut *mut c_void, *mut *mut c_void, *mut u32),
    PSGetConstantBuffers(u32, u32, *mut *mut c_void),
    IAGetInputLayout(*mut *mut c_void),
    IAGetVertexBuffers(u32, u32, *mut *mut c_void, *mut u32, *mut u32),
    IAGetIndexBuffer(*mut *mut c_void, *mut DXGI_FORMAT, *mut u32),
    GSGetConstantBuffers(u32, u32, *mut *mut c_void),
    GSGetShader(*mut *mut c_void, *mut *mut c_void, *mut u32),
    IAGetPrimitiveTopology(*mut D3D_PRIMITIVE_TOPOLOGY),
    VSGetShaderResources(u32, u32, *mut *mut c_void),
    VSGetSamplers(u32, u32, *mut *mut c_void),
    GetPredication(*mut *mut c_void, *mut BOOL),
    GSGetShaderResources(u32, u32, *mut *mut c_void),
    GSGetSamplers(u32, u32, *mut *mut c_void),
    OMGetRenderTargets(u32, *mut *mut c_void, *mut *mut c_void),
    OMGetRenderTargetsAndUnorderedAccessViews(u32, *mut *mut c_void, *mut *mut c_void, u32, u32, *mut *mut c_void),
    OMGetBlendState(*mut *mut c_void, *mut f32, *mut u32),
    OMGetDepthStencilState(*mut *mut c_void, *mut u32),
    SOGetTargets(u32, *mut *mut c_void),
    RSGetState(*mut *mut c_void),
    RSGetViewports(*mut u32, *mut D3D11_VIEWPORT),
    RSGetScissorRects(*mut u32, *mut RECT),
    HSGetShaderResources(u32, u32, *mut *mut c_void),
    HSGetShader(*mut *mut c_void, *mut *mut c_void, *mut u32),
    HSGetSamplers(u32, u32, *mut *mut c_void),
    HSGetConstantBuffers(u32, u32, *mut *mut c_void),
    DSGetShaderResources(u32, u32, *mut *mut c_void),
    DSGetShader(*mut *mut c_void, *mut *mut c_void, *mut u32),
    DSGetSamplers(u32, u32, *mut *mut c_void),
    DSGetConstantBuffers(u32, u32, *mut *mut c_void),
    CSGetShaderResources(u32, u32, *mut *mut c_void),
    CSGetUnorderedAccessViews(u32, u32, *mut *mut c_void),
    CSGetShader(*mut *mut c_void, *mut *mut c_void, *mut u32),
    CSGetSamplers(u32, u32, *mut *mut c_void),
    CSGetConstantBuffers(u32, u32, *mut *mut c_void),
    ClearState(),
    Flush(),
    GetType(),
    GetContextFlags(),
    FinishCommandList(BOOL, *mut *mut c_void),
}
impl Payload for D3DPayload {
    fn title(&self) -> String {
        match self {
            _ => self.to_string(),
        }
    }

    fn colour(&self) -> ig::Color {
        let type_index = D3DPayloadDiscriminants::from(self) as u32;
        let hue = type_index as f32 / D3DPayload::COUNT as f32;
        ig::Color::from_hsv(hue, 0.6, 0.8)
    }

    #[allow(non_snake_case)]
    #[rustfmt::skip]
    fn draw(&self) -> anyhow::Result<()> {
        match self {
            Self::QueryInterface(riid, ppvObject) => {
                ig::bulletf!("riid: {:?}", riid);
                ig::bulletf!("ppvObject: {:?}", ppvObject);
            }
            Self::GetDevice(ppDevice) => {
                ig::bulletf!("ppDevice: {:?}", ppDevice);
            }
            Self::GetPrivateData(guid, pDataSize, pData) => {
                ig::bulletf!("guid: {:?}", guid);
                ig::bulletf!("pDataSize: {:?}", pDataSize);
                ig::bulletf!("pData: {:?}", pData);
            }
            Self::SetPrivateData(guid, DataSize, pData) => {
                ig::bulletf!("guid: {:?}", guid);
                ig::bulletf!("DataSize: {:?}", DataSize);
                ig::bulletf!("pData: {:?}", pData);
            }
            Self::SetPrivateDataInterface(guid, pData) => {
                ig::bulletf!("guid: {:?}", guid);
                ig::bulletf!("pData: {:?}", pData);
            }
            Self::VSSetConstantBuffers(StartSlot, NumBuffers, ppConstantBuffers) => {
                ig::bulletf!("StartSlot: {:?}", StartSlot);
                ig::bulletf!("NumBuffers: {:?}", NumBuffers);
                ig::bulletf!("ppConstantBuffers: {:?}", ppConstantBuffers);
            }
            Self::PSSetShaderResources(StartSlot, NumViews, ppShaderResourceViews) => {
                ig::bulletf!("StartSlot: {:?}", StartSlot);
                ig::bulletf!("NumViews: {:?}", NumViews);
                ig::bulletf!("ppShaderResourceViews: {:?}", ppShaderResourceViews);
            }
            Self::PSSetShader(pPixelShader, ppClassInstances, NumClassInstances) => {
                ig::bulletf!("pPixelShader: {:?}", pPixelShader);
                ig::bulletf!("ppClassInstances: {:?}", ppClassInstances);
                ig::bulletf!("NumClassInstances: {:?}", NumClassInstances);
            }
            Self::PSSetSamplers(StartSlot, NumSamplers, ppSamplers) => {
                ig::bulletf!("StartSlot: {:?}", StartSlot);
                ig::bulletf!("NumSamplers: {:?}", NumSamplers);
                ig::bulletf!("ppSamplers: {:?}", ppSamplers);
            }
            Self::VSSetShader(pVertexShader, ppClassInstances, NumClassInstances) => {
                ig::bulletf!("pVertexShader: {:?}", pVertexShader);
                ig::bulletf!("ppClassInstances: {:?}", ppClassInstances);
                ig::bulletf!("NumClassInstances: {:?}", NumClassInstances);
            }
            Self::DrawIndexed(IndexCount, StartIndexLocation, BaseVertexLocation) => {
                ig::bulletf!("IndexCount: {:?}", IndexCount);
                ig::bulletf!("StartIndexLocation: {:?}", StartIndexLocation);
                ig::bulletf!("BaseVertexLocation: {:?}", BaseVertexLocation);
            }
            Self::Draw(VertexCount, StartVertexLocation) => {
                ig::bulletf!("VertexCount: {:?}", VertexCount);
                ig::bulletf!("StartVertexLocation: {:?}", StartVertexLocation);
            }
            Self::Map(pResource, Subresource, MapType, MapFlags, pMappedResource) => {
                ig::bulletf!("pResource: {:?}", pResource);
                ig::bulletf!("Subresource: {:?}", Subresource);
                ig::bulletf!("MapType: {:?}", MapType);
                ig::bulletf!("MapFlags: {:?}", MapFlags);
                ig::bulletf!("pMappedResource: {:?}", pMappedResource);
            }
            Self::Unmap(pResource, Subresource) => {
                ig::bulletf!("pResource: {:?}", pResource);
                ig::bulletf!("Subresource: {:?}", Subresource);
            }
            Self::PSSetConstantBuffers(StartSlot, NumBuffers, ppConstantBuffers) => {
                ig::bulletf!("StartSlot: {:?}", StartSlot);
                ig::bulletf!("NumBuffers: {:?}", NumBuffers);
                ig::bulletf!("ppConstantBuffers: {:?}", ppConstantBuffers);
            }
            Self::IASetInputLayout(pInputLayout) => {
                ig::bulletf!("pInputLayout: {:?}", pInputLayout);
            }
            Self::IASetVertexBuffers(StartSlot, NumBuffers, ppVertexBuffers, pStrides, pOffsets) => {
                ig::bulletf!("StartSlot: {:?}", StartSlot);
                ig::bulletf!("NumBuffers: {:?}", NumBuffers);
                ig::bulletf!("ppVertexBuffers: {:?}", ppVertexBuffers);
                ig::bulletf!("pStrides: {:?}", pStrides);
                ig::bulletf!("pOffsets: {:?}", pOffsets);
            }
            Self::IASetIndexBuffer(pIndexBuffer, Format, Offset) => {
                ig::bulletf!("pIndexBuffer: {:?}", pIndexBuffer);
                ig::bulletf!("Format: {:?}", Format);
                ig::bulletf!("Offset: {:?}", Offset);
            }
            Self::DrawIndexedInstanced(IndexCountPerInstance, InstanceCount, StartIndexLocation, BaseVertexLocation, StartInstanceLocation) => {
                ig::bulletf!("IndexCountPerInstance: {:?}", IndexCountPerInstance);
                ig::bulletf!("InstanceCount: {:?}", InstanceCount);
                ig::bulletf!("StartIndexLocation: {:?}", StartIndexLocation);
                ig::bulletf!("BaseVertexLocation: {:?}", BaseVertexLocation);
                ig::bulletf!("StartInstanceLocation: {:?}", StartInstanceLocation);
            }
            Self::DrawInstanced(VertexCountPerInstance, InstanceCount, StartVertexLocation, StartInstanceLocation) => {
                ig::bulletf!("VertexCountPerInstance: {:?}", VertexCountPerInstance);
                ig::bulletf!("InstanceCount: {:?}", InstanceCount);
                ig::bulletf!("StartVertexLocation: {:?}", StartVertexLocation);
                ig::bulletf!("StartInstanceLocation: {:?}", StartInstanceLocation);
            }
            Self::GSSetConstantBuffers(StartSlot, NumBuffers, ppConstantBuffers) => {
                ig::bulletf!("StartSlot: {:?}", StartSlot);
                ig::bulletf!("NumBuffers: {:?}", NumBuffers);
                ig::bulletf!("ppConstantBuffers: {:?}", ppConstantBuffers);
            }
            Self::GSSetShader(pShader, ppClassInstances, NumClassInstances) => {
                ig::bulletf!("pShader: {:?}", pShader);
                ig::bulletf!("ppClassInstances: {:?}", ppClassInstances);
                ig::bulletf!("NumClassInstances: {:?}", NumClassInstances);
            }
            Self::IASetPrimitiveTopology(Topology) => {
                ig::bulletf!("Topology: {:?}", Topology);
            }
            Self::VSSetShaderResources(StartSlot, NumViews, ppShaderResourceViews) => {
                ig::bulletf!("StartSlot: {:?}", StartSlot);
                ig::bulletf!("NumViews: {:?}", NumViews);
                ig::bulletf!("ppShaderResourceViews: {:?}", ppShaderResourceViews);
            }
            Self::VSSetSamplers(StartSlot, NumSamplers, ppSamplers) => {
                ig::bulletf!("StartSlot: {:?}", StartSlot);
                ig::bulletf!("NumSamplers: {:?}", NumSamplers);
                ig::bulletf!("ppSamplers: {:?}", ppSamplers);
            }
            Self::Begin(pAsync) => {
                ig::bulletf!("pAsync: {:?}", pAsync);
            }
            Self::End(pAsync) => {
                ig::bulletf!("pAsync: {:?}", pAsync);
            }
            Self::GetData(pAsync, pData, DataSize, GetDataFlags) => {
                ig::bulletf!("pAsync: {:?}", pAsync);
                ig::bulletf!("pData: {:?}", pData);
                ig::bulletf!("DataSize: {:?}", DataSize);
                ig::bulletf!("GetDataFlags: {:?}", GetDataFlags);
            }
            Self::SetPredication(pPredicate, PredicateValue) => {
                ig::bulletf!("pPredicate: {:?}", pPredicate);
                ig::bulletf!("PredicateValue: {:?}", PredicateValue);
            }
            Self::GSSetShaderResources(StartSlot, NumViews, ppShaderResourceViews) => {
                ig::bulletf!("StartSlot: {:?}", StartSlot);
                ig::bulletf!("NumViews: {:?}", NumViews);
                ig::bulletf!("ppShaderResourceViews: {:?}", ppShaderResourceViews);
            }
            Self::GSSetSamplers(StartSlot, NumSamplers, ppSamplers) => {
                ig::bulletf!("StartSlot: {:?}", StartSlot);
                ig::bulletf!("NumSamplers: {:?}", NumSamplers);
                ig::bulletf!("ppSamplers: {:?}", ppSamplers);
            }
            Self::OMSetRenderTargets(NumViews, ppRenderTargetViews, pDepthStencilView) => {
                ig::bulletf!("NumViews: {:?}", NumViews);
                ig::bulletf!("ppRenderTargetViews: {:?}", ppRenderTargetViews);
                ig::bulletf!("pDepthStencilView: {:?}", pDepthStencilView);
            }
            Self::OMSetRenderTargetsAndUnorderedAccessViews(NumRTVs, ppRenderTargetViews, pDepthStencilView, UAVStartSlot, NumUAVs, ppUnorderedAccessViews, pUAVInitialCounts) => {
                ig::bulletf!("NumRTVs: {:?}", NumRTVs);
                ig::bulletf!("ppRenderTargetViews: {:?}", ppRenderTargetViews);
                ig::bulletf!("pDepthStencilView: {:?}", pDepthStencilView);
                ig::bulletf!("UAVStartSlot: {:?}", UAVStartSlot);
                ig::bulletf!("NumUAVs: {:?}", NumUAVs);
                ig::bulletf!("ppUnorderedAccessViews: {:?}", ppUnorderedAccessViews);
                ig::bulletf!("pUAVInitialCounts: {:?}", pUAVInitialCounts);
            }
            Self::OMSetBlendState(pBlendState, BlendFactor, SampleMask) => {
                ig::bulletf!("pBlendState: {:?}", pBlendState);
                ig::bulletf!("BlendFactor: {:?}", BlendFactor);
                ig::bulletf!("SampleMask: {:?}", SampleMask);
            }
            Self::OMSetDepthStencilState(pDepthStencilState, StencilRef) => {
                ig::bulletf!("pDepthStencilState: {:?}", pDepthStencilState);
                ig::bulletf!("StencilRef: {:?}", StencilRef);
            }
            Self::SOSetTargets(NumBuffers, ppSOTargets, pOffsets) => {
                ig::bulletf!("NumBuffers: {:?}", NumBuffers);
                ig::bulletf!("ppSOTargets: {:?}", ppSOTargets);
                ig::bulletf!("pOffsets: {:?}", pOffsets);
            }
            Self::DrawIndexedInstancedIndirect(pBufferForArgs, AlignedByteOffsetForArgs) => {
                ig::bulletf!("pBufferForArgs: {:?}", pBufferForArgs);
                ig::bulletf!("AlignedByteOffsetForArgs: {:?}", AlignedByteOffsetForArgs);
            }
            Self::DrawInstancedIndirect(pBufferForArgs, AlignedByteOffsetForArgs) => {
                ig::bulletf!("pBufferForArgs: {:?}", pBufferForArgs);
                ig::bulletf!("AlignedByteOffsetForArgs: {:?}", AlignedByteOffsetForArgs);
            }
            Self::Dispatch(ThreadGroupCountX, ThreadGroupCountY, ThreadGroupCountZ) => {
                ig::bulletf!("ThreadGroupCountX: {:?}", ThreadGroupCountX);
                ig::bulletf!("ThreadGroupCountY: {:?}", ThreadGroupCountY);
                ig::bulletf!("ThreadGroupCountZ: {:?}", ThreadGroupCountZ);
            }
            Self::DispatchIndirect(pBufferForArgs, AlignedByteOffsetForArgs) => {
                ig::bulletf!("pBufferForArgs: {:?}", pBufferForArgs);
                ig::bulletf!("AlignedByteOffsetForArgs: {:?}", AlignedByteOffsetForArgs);
            }
            Self::RSSetState(pRasterizerState) => {
                ig::bulletf!("pRasterizerState: {:?}", pRasterizerState);
            }
            Self::RSSetViewports(NumViewports, pViewports) => {
                ig::bulletf!("NumViewports: {:?}", NumViewports);
                ig::bulletf!("pViewports: {:?}", pViewports);
            }
            Self::RSSetScissorRects(NumRects, pRects) => {
                ig::bulletf!("NumRects: {:?}", NumRects);
                ig::bulletf!("pRects: {:?}", pRects);
            }
            Self::CopySubresourceRegion(pDstResource, DstSubresource, DstX, DstY, DstZ, pSrcResource, SrcSubresource, pSrcBox) => {
                ig::bulletf!("pDstResource: {:?}", pDstResource);
                ig::bulletf!("DstSubresource: {:?}", DstSubresource);
                ig::bulletf!("DstX: {:?}", DstX);
                ig::bulletf!("DstY: {:?}", DstY);
                ig::bulletf!("DstZ: {:?}", DstZ);
                ig::bulletf!("pSrcResource: {:?}", pSrcResource);
                ig::bulletf!("SrcSubresource: {:?}", SrcSubresource);
                ig::bulletf!("pSrcBox: {:?}", pSrcBox);
            }
            Self::CopyResource(pDstResource, pSrcResource) => {
                ig::bulletf!("pDstResource: {:?}", pDstResource);
                ig::bulletf!("pSrcResource: {:?}", pSrcResource);
            }
            Self::UpdateSubresource(pDstResource, DstSubresource, pDstBox, pSrcData, SrcRowPitch, SrcDepthPitch) => {
                ig::bulletf!("pDstResource: {:?}", pDstResource);
                ig::bulletf!("DstSubresource: {:?}", DstSubresource);
                ig::bulletf!("pDstBox: {:?}", pDstBox);
                ig::bulletf!("pSrcData: {:?}", pSrcData);
                ig::bulletf!("SrcRowPitch: {:?}", SrcRowPitch);
                ig::bulletf!("SrcDepthPitch: {:?}", SrcDepthPitch);
            }
            Self::CopyStructureCount(pDstBuffer, DstAlignedByteOffset, pSrcView) => {
                ig::bulletf!("pDstBuffer: {:?}", pDstBuffer);
                ig::bulletf!("DstAlignedByteOffset: {:?}", DstAlignedByteOffset);
                ig::bulletf!("pSrcView: {:?}", pSrcView);
            }
            Self::ClearRenderTargetView(pRenderTargetView, ColorRGBA) => {
                ig::bulletf!("pRenderTargetView: {:?}", pRenderTargetView);
                ig::bulletf!("ColorRGBA: {:?}", ColorRGBA);
            }
            Self::ClearUnorderedAccessViewUint(pUnorderedAccessView, Values) => {
                ig::bulletf!("pUnorderedAccessView: {:?}", pUnorderedAccessView);
                ig::bulletf!("Values: {:?}", Values);
            }
            Self::ClearUnorderedAccessViewFloat(pUnorderedAccessView, Values) => {
                ig::bulletf!("pUnorderedAccessView: {:?}", pUnorderedAccessView);
                ig::bulletf!("Values: {:?}", Values);
            }
            Self::ClearDepthStencilView(pDepthStencilView, ClearFlags, Depth, Stencil) => {
                ig::bulletf!("pDepthStencilView: {:?}", pDepthStencilView);
                ig::bulletf!("ClearFlags: {:?}", ClearFlags);
                ig::bulletf!("Depth: {:?}", Depth);
                ig::bulletf!("Stencil: {:?}", Stencil);
            }
            Self::GenerateMips(pShaderResourceView) => {
                ig::bulletf!("pShaderResourceView: {:?}", pShaderResourceView);
            }
            Self::SetResourceMinLOD(pResource, MinLOD) => {
                ig::bulletf!("pResource: {:?}", pResource);
                ig::bulletf!("MinLOD: {:?}", MinLOD);
            }
            Self::GetResourceMinLOD(pResource) => {
                ig::bulletf!("pResource: {:?}", pResource);
            }
            Self::ResolveSubresource(pDstResource, DstSubresource, pSrcResource, SrcSubresource, Format) => {
                ig::bulletf!("pDstResource: {:?}", pDstResource);
                ig::bulletf!("DstSubresource: {:?}", DstSubresource);
                ig::bulletf!("pSrcResource: {:?}", pSrcResource);
                ig::bulletf!("SrcSubresource: {:?}", SrcSubresource);
                ig::bulletf!("Format: {:?}", Format);
            }
            Self::ExecuteCommandList(pCommandList, RestoreContextState) => {
                ig::bulletf!("pCommandList: {:?}", pCommandList);
                ig::bulletf!("RestoreContextState: {:?}", RestoreContextState);
            }
            Self::HSSetShaderResources(StartSlot, NumViews, ppShaderResourceViews) => {
                ig::bulletf!("StartSlot: {:?}", StartSlot);
                ig::bulletf!("NumViews: {:?}", NumViews);
                ig::bulletf!("ppShaderResourceViews: {:?}", ppShaderResourceViews);
            }
            Self::HSSetShader(pHullShader, ppClassInstances, NumClassInstances) => {
                ig::bulletf!("pHullShader: {:?}", pHullShader);
                ig::bulletf!("ppClassInstances: {:?}", ppClassInstances);
                ig::bulletf!("NumClassInstances: {:?}", NumClassInstances);
            }
            Self::HSSetSamplers(StartSlot, NumSamplers, ppSamplers) => {
                ig::bulletf!("StartSlot: {:?}", StartSlot);
                ig::bulletf!("NumSamplers: {:?}", NumSamplers);
                ig::bulletf!("ppSamplers: {:?}", ppSamplers);
            }
            Self::HSSetConstantBuffers(StartSlot, NumBuffers, ppConstantBuffers) => {
                ig::bulletf!("StartSlot: {:?}", StartSlot);
                ig::bulletf!("NumBuffers: {:?}", NumBuffers);
                ig::bulletf!("ppConstantBuffers: {:?}", ppConstantBuffers);
            }
            Self::DSSetShaderResources(StartSlot, NumViews, ppShaderResourceViews) => {
                ig::bulletf!("StartSlot: {:?}", StartSlot);
                ig::bulletf!("NumViews: {:?}", NumViews);
                ig::bulletf!("ppShaderResourceViews: {:?}", ppShaderResourceViews);
            }
            Self::DSSetShader(pDomainShader, ppClassInstances, NumClassInstances) => {
                ig::bulletf!("pDomainShader: {:?}", pDomainShader);
                ig::bulletf!("ppClassInstances: {:?}", ppClassInstances);
                ig::bulletf!("NumClassInstances: {:?}", NumClassInstances);
            }
            Self::DSSetSamplers(StartSlot, NumSamplers, ppSamplers) => {
                ig::bulletf!("StartSlot: {:?}", StartSlot);
                ig::bulletf!("NumSamplers: {:?}", NumSamplers);
                ig::bulletf!("ppSamplers: {:?}", ppSamplers);
            }
            Self::DSSetConstantBuffers(StartSlot, NumBuffers, ppConstantBuffers) => {
                ig::bulletf!("StartSlot: {:?}", StartSlot);
                ig::bulletf!("NumBuffers: {:?}", NumBuffers);
                ig::bulletf!("ppConstantBuffers: {:?}", ppConstantBuffers);
            }
            Self::CSSetShaderResources(StartSlot, NumViews, ppShaderResourceViews) => {
                ig::bulletf!("StartSlot: {:?}", StartSlot);
                ig::bulletf!("NumViews: {:?}", NumViews);
                ig::bulletf!("ppShaderResourceViews: {:?}", ppShaderResourceViews);
            }
            Self::CSSetUnorderedAccessViews(StartSlot, NumUAVs, ppUnorderedAccessViews, pUAVInitialCounts) => {
                ig::bulletf!("StartSlot: {:?}", StartSlot);
                ig::bulletf!("NumUAVs: {:?}", NumUAVs);
                ig::bulletf!("ppUnorderedAccessViews: {:?}", ppUnorderedAccessViews);
                ig::bulletf!("pUAVInitialCounts: {:?}", pUAVInitialCounts);
            }
            Self::CSSetShader(pComputeShader, ppClassInstances, NumClassInstances) => {
                ig::bulletf!("pComputeShader: {:?}", pComputeShader);
                ig::bulletf!("ppClassInstances: {:?}", ppClassInstances);
                ig::bulletf!("NumClassInstances: {:?}", NumClassInstances);
            }
            Self::CSSetSamplers(StartSlot, NumSamplers, ppSamplers) => {
                ig::bulletf!("StartSlot: {:?}", StartSlot);
                ig::bulletf!("NumSamplers: {:?}", NumSamplers);
                ig::bulletf!("ppSamplers: {:?}", ppSamplers);
            }
            Self::CSSetConstantBuffers(StartSlot, NumBuffers, ppConstantBuffers) => {
                ig::bulletf!("StartSlot: {:?}", StartSlot);
                ig::bulletf!("NumBuffers: {:?}", NumBuffers);
                ig::bulletf!("ppConstantBuffers: {:?}", ppConstantBuffers);
            }
            Self::VSGetConstantBuffers(StartSlot, NumBuffers, ppConstantBuffers) => {
                ig::bulletf!("StartSlot: {:?}", StartSlot);
                ig::bulletf!("NumBuffers: {:?}", NumBuffers);
                ig::bulletf!("ppConstantBuffers: {:?}", ppConstantBuffers);
            }
            Self::PSGetShaderResources(StartSlot, NumViews, ppShaderResourceViews) => {
                ig::bulletf!("StartSlot: {:?}", StartSlot);
                ig::bulletf!("NumViews: {:?}", NumViews);
                ig::bulletf!("ppShaderResourceViews: {:?}", ppShaderResourceViews);
            }
            Self::PSGetShader(ppPixelShader, ppClassInstances, pNumClassInstances) => {
                ig::bulletf!("ppPixelShader: {:?}", ppPixelShader);
                ig::bulletf!("ppClassInstances: {:?}", ppClassInstances);
                ig::bulletf!("pNumClassInstances: {:?}", pNumClassInstances);
            }
            Self::PSGetSamplers(StartSlot, NumSamplers, ppSamplers) => {
                ig::bulletf!("StartSlot: {:?}", StartSlot);
                ig::bulletf!("NumSamplers: {:?}", NumSamplers);
                ig::bulletf!("ppSamplers: {:?}", ppSamplers);
            }
            Self::VSGetShader(ppVertexShader, ppClassInstances, pNumClassInstances) => {
                ig::bulletf!("ppVertexShader: {:?}", ppVertexShader);
                ig::bulletf!("ppClassInstances: {:?}", ppClassInstances);
                ig::bulletf!("pNumClassInstances: {:?}", pNumClassInstances);
            }
            Self::PSGetConstantBuffers(StartSlot, NumBuffers, ppConstantBuffers) => {
                ig::bulletf!("StartSlot: {:?}", StartSlot);
                ig::bulletf!("NumBuffers: {:?}", NumBuffers);
                ig::bulletf!("ppConstantBuffers: {:?}", ppConstantBuffers);
            }
            Self::IAGetInputLayout(ppInputLayout) => {
                ig::bulletf!("ppInputLayout: {:?}", ppInputLayout);
            }
            Self::IAGetVertexBuffers(StartSlot, NumBuffers, ppVertexBuffers, pStrides, pOffsets) => {
                ig::bulletf!("StartSlot: {:?}", StartSlot);
                ig::bulletf!("NumBuffers: {:?}", NumBuffers);
                ig::bulletf!("ppVertexBuffers: {:?}", ppVertexBuffers);
                ig::bulletf!("pStrides: {:?}", pStrides);
                ig::bulletf!("pOffsets: {:?}", pOffsets);
            }
            Self::IAGetIndexBuffer(pIndexBuffer, Format, Offset) => {
                ig::bulletf!("pIndexBuffer: {:?}", pIndexBuffer);
                ig::bulletf!("Format: {:?}", Format);
                ig::bulletf!("Offset: {:?}", Offset);
            }
            Self::GSGetConstantBuffers(StartSlot, NumBuffers, ppConstantBuffers) => {
                ig::bulletf!("StartSlot: {:?}", StartSlot);
                ig::bulletf!("NumBuffers: {:?}", NumBuffers);
                ig::bulletf!("ppConstantBuffers: {:?}", ppConstantBuffers);
            }
            Self::GSGetShader(ppGeometryShader, ppClassInstances, pNumClassInstances) => {
                ig::bulletf!("ppGeometryShader: {:?}", ppGeometryShader);
                ig::bulletf!("ppClassInstances: {:?}", ppClassInstances);
                ig::bulletf!("pNumClassInstances: {:?}", pNumClassInstances);
            }
            Self::IAGetPrimitiveTopology(pTopology) => {
                ig::bulletf!("pTopology: {:?}", pTopology);
            }
            Self::VSGetShaderResources(StartSlot, NumViews, ppShaderResourceViews) => {
                ig::bulletf!("StartSlot: {:?}", StartSlot);
                ig::bulletf!("NumViews: {:?}", NumViews);
                ig::bulletf!("ppShaderResourceViews: {:?}", ppShaderResourceViews);
            }
            Self::VSGetSamplers(StartSlot, NumSamplers, ppSamplers) => {
                ig::bulletf!("StartSlot: {:?}", StartSlot);
                ig::bulletf!("NumSamplers: {:?}", NumSamplers);
                ig::bulletf!("ppSamplers: {:?}", ppSamplers);
            }
            Self::GetPredication(ppPredicate, pPredicateValue) => {
                ig::bulletf!("ppPredicate: {:?}", ppPredicate);
                ig::bulletf!("pPredicateValue: {:?}", pPredicateValue);
            }
            Self::GSGetShaderResources(StartSlot, NumViews, ppShaderResourceViews) => {
                ig::bulletf!("StartSlot: {:?}", StartSlot);
                ig::bulletf!("NumViews: {:?}", NumViews);
                ig::bulletf!("ppShaderResourceViews: {:?}", ppShaderResourceViews);
            }
            Self::GSGetSamplers(StartSlot, NumSamplers, ppSamplers) => {
                ig::bulletf!("StartSlot: {:?}", StartSlot);
                ig::bulletf!("NumSamplers: {:?}", NumSamplers);
                ig::bulletf!("ppSamplers: {:?}", ppSamplers);
            }
            Self::OMGetRenderTargets(NumViews, ppRenderTargetViews, ppDepthStencilView) => {
                ig::bulletf!("NumViews: {:?}", NumViews);
                ig::bulletf!("ppRenderTargetViews: {:?}", ppRenderTargetViews);
                ig::bulletf!("ppDepthStencilView: {:?}", ppDepthStencilView);
            }
            Self::OMGetRenderTargetsAndUnorderedAccessViews(NumRTVs, ppRenderTargetViews, ppDepthStencilView, UAVStartSlot, NumUAVs, ppUnorderedAccessViews) => {
                ig::bulletf!("NumRTVs: {:?}", NumRTVs);
                ig::bulletf!("ppRenderTargetViews: {:?}", ppRenderTargetViews);
                ig::bulletf!("ppDepthStencilView: {:?}", ppDepthStencilView);
                ig::bulletf!("UAVStartSlot: {:?}", UAVStartSlot);
                ig::bulletf!("NumUAVs: {:?}", NumUAVs);
                ig::bulletf!("ppUnorderedAccessViews: {:?}", ppUnorderedAccessViews);
            }
            Self::OMGetBlendState(ppBlendState, BlendFactor, pSampleMask) => {
                ig::bulletf!("ppBlendState: {:?}", ppBlendState);
                ig::bulletf!("BlendFactor: {:?}", BlendFactor);
                ig::bulletf!("pSampleMask: {:?}", pSampleMask);
            }
            Self::OMGetDepthStencilState(ppDepthStencilState, pStencilRef) => {
                ig::bulletf!("ppDepthStencilState: {:?}", ppDepthStencilState);
                ig::bulletf!("pStencilRef: {:?}", pStencilRef);
            }
            Self::SOGetTargets(NumBuffers, ppSOTargets) => {
                ig::bulletf!("NumBuffers: {:?}", NumBuffers);
                ig::bulletf!("ppSOTargets: {:?}", ppSOTargets);
            }
            Self::RSGetState(ppRasterizerState) => {
                ig::bulletf!("ppRasterizerState: {:?}", ppRasterizerState);
            }
            Self::RSGetViewports(pNumViewports, pViewports) => {
                ig::bulletf!("pNumViewports: {:?}", pNumViewports);
                ig::bulletf!("pViewports: {:?}", pViewports);
            }
            Self::RSGetScissorRects(pNumRects, pRects) => {
                ig::bulletf!("pNumRects: {:?}", pNumRects);
                ig::bulletf!("pRects: {:?}", pRects);
            }
            Self::HSGetShaderResources(StartSlot, NumViews, ppShaderResourceViews) => {
                ig::bulletf!("StartSlot: {:?}", StartSlot);
                ig::bulletf!("NumViews: {:?}", NumViews);
                ig::bulletf!("ppShaderResourceViews: {:?}", ppShaderResourceViews);
            }
            Self::HSGetShader(ppHullShader, ppClassInstances, pNumClassInstances) => {
                ig::bulletf!("ppHullShader: {:?}", ppHullShader);
                ig::bulletf!("ppClassInstances: {:?}", ppClassInstances);
                ig::bulletf!("pNumClassInstances: {:?}", pNumClassInstances);
            }
            Self::HSGetSamplers(StartSlot, NumSamplers, ppSamplers) => {
                ig::bulletf!("StartSlot: {:?}", StartSlot);
                ig::bulletf!("NumSamplers: {:?}", NumSamplers);
                ig::bulletf!("ppSamplers: {:?}", ppSamplers);
            }
            Self::HSGetConstantBuffers(StartSlot, NumBuffers, ppConstantBuffers) => {
                ig::bulletf!("StartSlot: {:?}", StartSlot);
                ig::bulletf!("NumBuffers: {:?}", NumBuffers);
                ig::bulletf!("ppConstantBuffers: {:?}", ppConstantBuffers);
            }
            Self::DSGetShaderResources(StartSlot, NumViews, ppShaderResourceViews) => {
                ig::bulletf!("StartSlot: {:?}", StartSlot);
                ig::bulletf!("NumViews: {:?}", NumViews);
                ig::bulletf!("ppShaderResourceViews: {:?}", ppShaderResourceViews);
            }
            Self::DSGetShader(ppDomainShader, ppClassInstances, pNumClassInstances) => {
                ig::bulletf!("ppDomainShader: {:?}", ppDomainShader);
                ig::bulletf!("ppClassInstances: {:?}", ppClassInstances);
                ig::bulletf!("pNumClassInstances: {:?}", pNumClassInstances);
            }
            Self::DSGetSamplers(StartSlot, NumSamplers, ppSamplers) => {
                ig::bulletf!("StartSlot: {:?}", StartSlot);
                ig::bulletf!("NumSamplers: {:?}", NumSamplers);
                ig::bulletf!("ppSamplers: {:?}", ppSamplers);
            }
            Self::DSGetConstantBuffers(StartSlot, NumBuffers, ppConstantBuffers) => {
                ig::bulletf!("StartSlot: {:?}", StartSlot);
                ig::bulletf!("NumBuffers: {:?}", NumBuffers);
                ig::bulletf!("ppConstantBuffers: {:?}", ppConstantBuffers);
            }
            Self::CSGetShaderResources(StartSlot, NumViews, ppShaderResourceViews) => {
                ig::bulletf!("StartSlot: {:?}", StartSlot);
                ig::bulletf!("NumViews: {:?}", NumViews);
                ig::bulletf!("ppShaderResourceViews: {:?}", ppShaderResourceViews);
            }
            Self::CSGetUnorderedAccessViews(StartSlot, NumUAVs, ppUnorderedAccessViews) => {
                ig::bulletf!("StartSlot: {:?}", StartSlot);
                ig::bulletf!("NumUAVs: {:?}", NumUAVs);
                ig::bulletf!("ppUnorderedAccessViews: {:?}", ppUnorderedAccessViews);
            }
            Self::CSGetShader(ppComputeShader, ppClassInstances, pNumClassInstances) => {
                ig::bulletf!("ppComputeShader: {:?}", ppComputeShader);
                ig::bulletf!("ppClassInstances: {:?}", ppClassInstances);
                ig::bulletf!("pNumClassInstances: {:?}", pNumClassInstances);
            }
            Self::CSGetSamplers(StartSlot, NumSamplers, ppSamplers) => {
                ig::bulletf!("StartSlot: {:?}", StartSlot);
                ig::bulletf!("NumSamplers: {:?}", NumSamplers);
                ig::bulletf!("ppSamplers: {:?}", ppSamplers);
            }
            Self::CSGetConstantBuffers(StartSlot, NumBuffers, ppConstantBuffers) => {
                ig::bulletf!("StartSlot: {:?}", StartSlot);
                ig::bulletf!("NumBuffers: {:?}", NumBuffers);
                ig::bulletf!("ppConstantBuffers: {:?}", ppConstantBuffers);
            }
            Self::FinishCommandList(RestoreDeferredContextState, ppCommandList) => {
                ig::bulletf!("RestoreDeferredContextState: {:?}", RestoreDeferredContextState);
                ig::bulletf!("ppCommandList: {:?}", ppCommandList);
            }
            _ => {}
        }

        Ok(())
    }
}

pub type D3DCommand = Command<D3DPayload>;