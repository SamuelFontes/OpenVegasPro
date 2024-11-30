#pragma once
#include <string>
#include <regex>
#include "enums.h"
#include "utils.h"
#include <raylib.h>

class MediaSource{
private:
	std::string m_mediaPath;
    std::string m_fileName;
	MediaType m_mediaType;
    Texture2D m_miniature;

public:
    MediaSource(const std::string& filePath);
    ~MediaSource();
    char* GetFileName();
};