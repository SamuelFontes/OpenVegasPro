#pragma once
#include <string>
#include <regex>
#include "Enums.h"
#include "Utils.h"
#include <raylib.h>

class MediaSource{
private:
	std::string m_mediaPath = {};
    std::string m_fileName = {};
    double m_fps = 0;
	MediaType m_mediaType = {};
    Texture2D m_miniature = {};

public:
    MediaSource(const std::string& filePath);
    ~MediaSource();
    char* GetFileName();
};