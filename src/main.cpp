// OpenVegasPro.cpp : Defines the entry point for the application.
//

#include "OpenVegasPro.h"
#include <QApplication>
#include <QDebug>
#include <QtWidgets>


using namespace std;

int main(int argc, char** argv)
{
	QApplication app(argc, argv);
	QWidget window;
	window.resize(320, 240);
	window.show();
	window.setWindowTitle(
		QApplication::translate("toplevel", "Top-level widget"));
	return app.exec();
}
