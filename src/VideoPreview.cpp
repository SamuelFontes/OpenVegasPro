#include "VideoPreview.h"

void VideoPreview::RenderFrame()
{
	if (_hasFrameToRender) {
		DrawTexturePro((*_currentFrame), { 0,0,(float)(*_currentFrame).width,(float)(*_currentFrame).height }, _renderingRectangle, { 0,0 }, 0, WHITE);
	}
	else {
		// If there is nothing to render just render a black rectangle
		DrawRectangleRec(_renderingRectangle,BLACK);
	}
}

void VideoPreview::SetCurrentFrame(Texture2D* frame)
{
	_currentFrame = frame;
	_hasFrameToRender = true;
}

void VideoPreview::UnloadCurrentFrame()
{
	_hasFrameToRender = false;
	_currentFrame = nullptr; // Is this a good thing?
}

