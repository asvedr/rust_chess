import Tkinter

figs = 'kqbhtp'
lits = 'ABCDEFGH'

class Figure:
	MUL = 1
	def __init__(self, color, kind, cell):
		if type(cell) == str:
			self.x = lits.index(cell[0])
			self.y = int(cell[1])
		else:
			self.x = cell[0]
			self.y = cell[1]
		self.color = color
		self.colorBG = 'black' if color == 'white' else 'white'
		self.kind = kind
	def draw(self,canvas):
		MUL = Figure.MUL
		h = MUL / 2.0
		x = (self.x + 1) * MUL - h
		y = self.y * MUL - h
		r = MUL * 0.45
		canvas.create_oval(x - r, y - r, x + r, y + r, fill=self.colorBG)
		canvas.create_text(x, y, text=self.kind.upper(), fill=self.color, font=('Monospace', '20'))
	def json(self):
		return [figs.index(self.kind), lits[self.x] + str(self.y)]
