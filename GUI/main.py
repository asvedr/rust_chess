from field import *
from figures import *

def main():
	Figure.MUL = 30
	Field.MUL = 30
	root = Tk()
	Field(root, 'target/debug/chess').pack()
	root.mainloop()

main()
