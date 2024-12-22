#include "media_source.h"

MediaSource::MediaSource(const std::string& filePath)
{
    // Get media type
    if(Utils::StringContainAny(filePath, { ".mp4", ".mkv" })) // TODO: add all supported formats here, or move it elsewhere
    {
        m_mediaType = MediaType::Video;
		// Get miniature texture
		cv::VideoCapture capture = cv::VideoCapture(filePath);
		cv::Mat frame;
		capture >> frame;

		m_miniature = Utils::GetTextureFromVideoFrame(frame); // grabs the first frame

        m_fps = capture.get(cv::CAP_PROP_FPS); // Grab video fps

		capture.release();
		frame.release();
    }
    else if(Utils::StringContainAny(filePath, { ".mp3", ".obb" }))
    {
        m_mediaType = MediaType::Audio;
    }
    else if(Utils::StringContainAny(filePath, { ".png", ".jpg", ".jpeg" }))
    {
        m_mediaType = MediaType::Image;
    }
    else
    {
        m_mediaType = MediaType::Unknown;
    }
    
    // get fileName
    std::regex getFileNamePattern("[^/]*$"); 
    std::smatch result;
    if(std::regex_search(filePath, result, getFileNamePattern)){
        m_fileName = result[0];
    }
    else m_fileName = filePath; // just in case my regex doesn't work
    
}

MediaSource::~MediaSource()
{
    UnloadTexture(m_miniature);
}

char* MediaSource::GetFileName()
{
    return m_fileName.data();
}
