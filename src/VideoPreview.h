#pragma once
#include "raylib.h"

class VideoPreview {
private:
	bool hasFrameToRender;
	Texture2D* currentFrame;
	Rectangle renderingRectangle;

public:
	VideoPreview(Rectangle rec) {
		hasFrameToRender = false;
		currentFrame = nullptr;
		renderingRectangle = rec;
	}
	void RenderFrame();
	void SetCurrentFrame(Texture2D* frame);
	void UnloadCurrentFrame();
};
