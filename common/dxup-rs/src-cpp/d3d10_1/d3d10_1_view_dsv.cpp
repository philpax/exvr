#include "d3d10_1_device.h"
#include "d3d10_1_buffer.h"
#include "d3d10_1_texture.h"
#include "d3d10_1_view_dsv.h"

namespace dxup
{
	D3D10DepthStencilView::D3D10DepthStencilView(const D3D10_DEPTH_STENCIL_VIEW_DESC* pDesc, D3D10Device* pDevice, ID3D11DepthStencilView* pDSV)
		: m_cachedResource10(nullptr)
		, m_cachedResource11(nullptr)
	{
		if (pDesc)
			this->m_desc = *pDesc;

		this->m_device = pDevice;
		this->SetBase(pDSV);
	}

	HRESULT STDMETHODCALLTYPE D3D10DepthStencilView::QueryInterface(REFIID riid, void** ppvObject)
	{
		*ppvObject = nullptr;

		if (riid == __uuidof(IUnknown)
			|| riid == __uuidof(ID3D10DeviceChild)
			|| riid == __uuidof(ID3D10View)
			|| riid == __uuidof(ID3D10DepthStencilView)) {
			this->AddRef();
			*ppvObject = this;
			return S_OK;
		}

		DXUP_Log(Warn, "Couldn't find interface!");
		return E_FAIL;
	}

	void STDMETHODCALLTYPE D3D10DepthStencilView::GetResource(ID3D10Resource** ppResource)
	{
		GetBaseResource(ppResource, this);
	}
}
