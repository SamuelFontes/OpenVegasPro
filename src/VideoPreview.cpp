#include "VideoPreview.h"

void VideoPreview::RenderFrame()
{
	if (hasFrameToRender) {
		DrawTexturePro((*currentFrame), { 0,0,(float)(*currentFrame).width,(float)(*currentFrame).height }, renderingRectangle, { 0,0 }, 0, WHITE);
	}
	else {
		// If there is nothing to render just render a black rectangle
		DrawRectangleRec(renderingRectangle,BLACK);
	}
}

void VideoPreview::SetCurrentFrame(Texture2D* frame)
{
	currentFrame = frame;
	hasFrameToRender = true;
}

void VideoPreview::UnloadCurrentFrame()
{
	hasFrameToRender = false;
	currentFrame = nullptr; // Is this a good thing?
}

