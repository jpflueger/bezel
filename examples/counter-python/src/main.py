from bezel import App, Box, Text, Button, Input, run

class Counter(App):
    def build(self):
        self.n = 0
        self.out = Text("0")
        self.name = Input(placeholder="your name")
        return Box(style={"direction": "column", "gap": 8, "padding": 16}, children=[
            self.name, self.out, Button("+1", on_click=self.bump),
        ])

    async def bump(self, ev):
        self.n += 1
        self.out.text = f"{self.name.value} · {self.n}"   # batched into one apply()

run(Counter)
