from Tkinter import *
from subprocess import check_output
from figures import *

class AIExcept(Exception):
	def __init__(self, val):
		Exception.__init__(self, 'AIExcept', val)
		self.val = val


class Field(Canvas):
	MUL = 1
	def __init__(self, root, aiPath):
		Canvas.__init__(self, root, width=(Field.MUL * 8), height=(Field.MUL * 8))
		self.aiPath = aiPath
		initFig = self.aiRequest('-rp -of js')
		self.white = [Figure('white', f[0], f[1]) for f in initFig['white']]
		self.black = [Figure('black', f[0], f[1]) for f in initFig['black']]
		self.redraw()
	def aiRequest(self, args):
		ans = check_output([self.aiPath] + args.split(' '))
		js = eval(ans)
		if 'error' in js:
			print js['error']
			raise AIExcept(js['error'])
		return js
	def redraw(self):
		self.delete(ALL)
		self.drawGrid()
		for fig in self.white:
			fig.draw(self)
		for fig in self.black:
			fig.draw(self)
	def drawGrid(self):
		MUL = Field.MUL
		for x in range(8):
			for y in range(8):
				if x % 2 == y % 2:
					self.create_rectangle(x * MUL, y * MUL, (x+1) * MUL, (y+1) * MUL, fill='black')
