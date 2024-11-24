#pragma once
#include <string>
#include <vector>


class Utils
{
public:
	static bool StringContainAny(const std::string& source, std::vector<std::string> matches);
};