#pragma once
#include "raylib.h"

class VideoPreview {
private:
	bool _hasFrameToRender;
	Texture2D* _currentFrame;
	Rectangle _renderingRectangle;

public:
	VideoPreview(Rectangle rec) {
		_hasFrameToRender = false;
		_currentFrame = nullptr;
		_renderingRectangle = rec;
	}
	void RenderFrame();
	void SetCurrentFrame(Texture2D* frame);
	void UnloadCurrentFrame();
};
