#include "VideoPreview.h"

void VideoPreview::RenderFrame()
{
	if (m_hasFrameToRender) {
		DrawTexturePro((*m_currentFrame), { 0,0,(float)(*m_currentFrame).width,(float)(*m_currentFrame).height }, m_renderingRectangle, { 0,0 }, 0, WHITE);
	}
	else {
		// If there is nothing to render just render a black rectangle
		DrawRectangleRec(m_renderingRectangle, BLACK);
	}
}

void VideoPreview::SetCurrentFrame(Texture2D* frame)
{
	m_currentFrame = frame;
	m_hasFrameToRender = true;
}

void VideoPreview::UnloadCurrentFrame()
{
	m_hasFrameToRender = false;
	m_currentFrame = nullptr; // Is this a good thing?
}

VideoPreview::VideoPreview(Rectangle rec)
{
	m_hasFrameToRender = false;
	m_currentFrame = nullptr;
	m_renderingRectangle = rec;
}
