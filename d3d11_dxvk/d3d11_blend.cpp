#include "d3d11_blend.h"
#include "d3d11_device.h"

namespace dxvk {

  D3D11BlendState::D3D11BlendState(
          D3D11Device*        device,
    const D3D11_BLEND_DESC1&  desc)
  : D3D11StateObject<ID3D11BlendState1>(device),
    m_desc(desc), m_d3d10(this) {
  }


  D3D11BlendState::~D3D11BlendState() {

  }


  HRESULT STDMETHODCALLTYPE D3D11BlendState::QueryInterface(REFIID riid, void** ppvObject) {
    if (ppvObject == nullptr)
      return E_POINTER;

    *ppvObject = nullptr;

    if (riid == __uuidof(IUnknown)
     || riid == __uuidof(ID3D11DeviceChild)
     || riid == __uuidof(ID3D11BlendState)
     || riid == __uuidof(ID3D11BlendState1)) {
      *ppvObject = ref(this);
      return S_OK;
    }

    if (riid == __uuidof(ID3D10DeviceChild)
     || riid == __uuidof(ID3D10BlendState)
     || riid == __uuidof(ID3D10BlendState1)) {
      *ppvObject = ref(&m_d3d10);
      return S_OK;
    }

    Logger::warn("D3D11BlendState::QueryInterface: Unknown interface query");
    Logger::warn(str::format(riid));
    return E_NOINTERFACE;
  }


  void STDMETHODCALLTYPE D3D11BlendState::GetDesc(D3D11_BLEND_DESC* pDesc) {
    pDesc->AlphaToCoverageEnable  = m_desc.AlphaToCoverageEnable;
    pDesc->IndependentBlendEnable = m_desc.IndependentBlendEnable;

    for (uint32_t i = 0; i < 8; i++) {
      pDesc->RenderTarget[i].BlendEnable           = m_desc.RenderTarget[i].BlendEnable;
      pDesc->RenderTarget[i].SrcBlend              = m_desc.RenderTarget[i].SrcBlend;
      pDesc->RenderTarget[i].DestBlend             = m_desc.RenderTarget[i].DestBlend;
      pDesc->RenderTarget[i].BlendOp               = m_desc.RenderTarget[i].BlendOp;
      pDesc->RenderTarget[i].SrcBlendAlpha         = m_desc.RenderTarget[i].SrcBlendAlpha;
      pDesc->RenderTarget[i].DestBlendAlpha        = m_desc.RenderTarget[i].DestBlendAlpha;
      pDesc->RenderTarget[i].BlendOpAlpha          = m_desc.RenderTarget[i].BlendOpAlpha;
      pDesc->RenderTarget[i].RenderTargetWriteMask = m_desc.RenderTarget[i].RenderTargetWriteMask;
    }
  }


  void STDMETHODCALLTYPE D3D11BlendState::GetDesc1(D3D11_BLEND_DESC1* pDesc) {
    *pDesc = m_desc;
  }


  D3D11_BLEND_DESC1 D3D11BlendState::PromoteDesc(const D3D11_BLEND_DESC* pSrcDesc) {
    D3D11_BLEND_DESC1 dstDesc;
    dstDesc.AlphaToCoverageEnable  = pSrcDesc->AlphaToCoverageEnable;
    dstDesc.IndependentBlendEnable = pSrcDesc->IndependentBlendEnable;

    for (uint32_t i = 0; i < 8; i++) {
      dstDesc.RenderTarget[i].BlendEnable           = pSrcDesc->RenderTarget[i].BlendEnable;
      dstDesc.RenderTarget[i].LogicOpEnable         = FALSE;
      dstDesc.RenderTarget[i].SrcBlend              = pSrcDesc->RenderTarget[i].SrcBlend;
      dstDesc.RenderTarget[i].DestBlend             = pSrcDesc->RenderTarget[i].DestBlend;
      dstDesc.RenderTarget[i].BlendOp               = pSrcDesc->RenderTarget[i].BlendOp;
      dstDesc.RenderTarget[i].SrcBlendAlpha         = pSrcDesc->RenderTarget[i].SrcBlendAlpha;
      dstDesc.RenderTarget[i].DestBlendAlpha        = pSrcDesc->RenderTarget[i].DestBlendAlpha;
      dstDesc.RenderTarget[i].BlendOpAlpha          = pSrcDesc->RenderTarget[i].BlendOpAlpha;
      dstDesc.RenderTarget[i].LogicOp               = D3D11_LOGIC_OP_NOOP;
      dstDesc.RenderTarget[i].RenderTargetWriteMask = pSrcDesc->RenderTarget[i].RenderTargetWriteMask;
    }

    return dstDesc;
  }


  HRESULT D3D11BlendState::NormalizeDesc(D3D11_BLEND_DESC1* pDesc) {
    if (pDesc->AlphaToCoverageEnable)
      pDesc->AlphaToCoverageEnable = TRUE;

    if (pDesc->IndependentBlendEnable)
      pDesc->IndependentBlendEnable = TRUE;

    const uint32_t numRenderTargets = pDesc->IndependentBlendEnable ? 8 : 1;

    for (uint32_t i = 0; i < numRenderTargets; i++) {
      D3D11_RENDER_TARGET_BLEND_DESC1* rt = &pDesc->RenderTarget[i];

      if (rt->BlendEnable) {
        rt->BlendEnable = TRUE;

        if (rt->LogicOpEnable)
          return E_INVALIDARG;

        if (!ValidateBlendOperations(
         rt->SrcBlend, rt->SrcBlendAlpha,
         rt->DestBlend, rt->DestBlendAlpha,
         rt->BlendOp, rt->BlendOpAlpha))
          return E_INVALIDARG;
      } else {
        rt->SrcBlend       = D3D11_BLEND_ONE;
        rt->DestBlend      = D3D11_BLEND_ZERO;
        rt->BlendOp        = D3D11_BLEND_OP_ADD;
        rt->SrcBlendAlpha  = D3D11_BLEND_ONE;
        rt->DestBlendAlpha = D3D11_BLEND_ZERO;
        rt->BlendOpAlpha   = D3D11_BLEND_OP_ADD;
      }

      if (rt->LogicOpEnable) {
        rt->LogicOpEnable = TRUE;

        // Blending must be disabled
        // if the logic op is enabled
        if (rt->BlendEnable
         || pDesc->IndependentBlendEnable
         || !ValidateLogicOp(rt->LogicOp))
          return E_INVALIDARG;
      } else {
        rt->LogicOp = D3D11_LOGIC_OP_NOOP;
      }

      if (rt->RenderTargetWriteMask > D3D11_COLOR_WRITE_ENABLE_ALL)
        return E_INVALIDARG;
    }

    for (uint32_t i = numRenderTargets; i < 8; i++) {
      // Render targets blend operations are the same
      // across all render targets when blend is enabled
      // on rendertarget[0] with independent blend disabled
      pDesc->RenderTarget[i] = pDesc->RenderTarget[0];
    }

    return S_OK;
  }

  bool D3D11BlendState::ValidateBlendFactor(D3D11_BLEND Blend) {
    return Blend >= D3D11_BLEND_ZERO
        && Blend <= D3D11_BLEND_INV_SRC1_ALPHA;
  }


  bool D3D11BlendState::ValidateBlendFactorAlpha(D3D11_BLEND BlendAlpha) {
    return BlendAlpha >= D3D11_BLEND_ZERO
        && BlendAlpha <= D3D11_BLEND_INV_SRC1_ALPHA
        && BlendAlpha != D3D11_BLEND_SRC_COLOR
        && BlendAlpha != D3D11_BLEND_INV_SRC_COLOR
        && BlendAlpha != D3D11_BLEND_DEST_COLOR
        && BlendAlpha != D3D11_BLEND_INV_DEST_COLOR
        && BlendAlpha != D3D11_BLEND_SRC1_COLOR
        && BlendAlpha != D3D11_BLEND_INV_SRC1_COLOR;
  }


  bool D3D11BlendState::ValidateBlendOp(D3D11_BLEND_OP BlendOp) {
    return BlendOp >= D3D11_BLEND_OP_ADD
        && BlendOp <= D3D11_BLEND_OP_MAX;
  }


  bool D3D11BlendState::ValidateLogicOp(D3D11_LOGIC_OP LogicOp) {
    return LogicOp >= D3D11_LOGIC_OP_CLEAR
        && LogicOp <= D3D11_LOGIC_OP_OR_INVERTED;
  }


  bool D3D11BlendState::ValidateBlendOperations(
          D3D11_BLEND     SrcBlend,
          D3D11_BLEND     SrcBlendAlpha,
          D3D11_BLEND     DestBlend,
          D3D11_BLEND     DestBlendAlpha,
          D3D11_BLEND_OP  BlendOp,
          D3D11_BLEND_OP  BlendOpAlpha) {
    return ValidateBlendOp(BlendOp)
        && ValidateBlendOp(BlendOpAlpha)
        && ValidateBlendFactor(SrcBlend)
        && ValidateBlendFactor(DestBlend)
        && ValidateBlendFactorAlpha(SrcBlendAlpha)
        && ValidateBlendFactorAlpha(DestBlendAlpha);
  }

}
