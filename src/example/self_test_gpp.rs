import rfile.h

void main()
{
	rf.cmd("gpp_build ..\\src\\rush.cxx")
	rf.cmd("copy /B /Y ..\\src\\rush.exe rush_gpp.exe")
	rf.cmd("mingw_test")
	rf.cmd("rush_mingw -nasm ../src/rush.cxx")
	rf.cmd("copy /B /Y ..\\src\\rush.asm rush_gpp.asm")
	rf.cmd("rush_gpp -nasm ../src/rush.cxx")
	if rfile.read_all_n("rush_gpp.asm")==rfile.read_all_n("..\\src\\rush.asm")
		printl "ok"
	else
		printl "error"
}
