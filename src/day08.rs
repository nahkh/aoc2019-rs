#[derive(Debug)]
struct Layer {
    width: usize,
    height: usize,
    data: Vec<u8>,
}

trait CharacterCounter {
    fn count_char(&self, value: u8) -> usize;
}

impl CharacterCounter for Layer {
    fn count_char(&self, value: u8) -> usize {
        let mut count = 0;
        for c in self.data.iter() {
            if *c == value {
                count += 1;
            }
        }

        count
    }
}

#[derive(Debug)]
struct Image {
    width: usize,
    height: usize,
    layers: Vec<Layer>,
}

trait CharacterConsumer {
    fn accept(&mut self, value: u8);
}

impl CharacterConsumer for Image {
    fn accept(&mut self, value: u8) {
        if self.layers.len() == 0
            || self.layers[self.layers.len() - 1].data.len() >= self.width * self.height
        {
            self.layers.push(Layer {
                width: self.width,
                height: self.height,
                data: Vec::new(),
            });
        }
        let last_index = self.layers.len() - 1;
        self.layers[last_index].data.push(value);
    }
}

trait LayerProducer {
    fn read_value(&self, index: usize) -> u8;
    fn produce_layer(&self) -> Layer;
}

impl LayerProducer for Image {
    fn read_value(&self, index: usize) -> u8 {
        for layer in self.layers.iter() {
            if layer.data[index] != 2 {
                return layer.data[index];
            }
        }
        2
    }

    fn produce_layer(&self) -> Layer {
        let mut layer = Layer {
            width: self.width,
            height: self.height,
            data: Vec::new(),
        };

        for i in 0..(self.width * self.height) {
            layer.data.push(self.read_value(i));
        }

        layer
    }
}

fn get_char(value: u8) -> char {
    match value {
        0 => ' ',
        1 => '#',
        2 => '2',
        _ => '?',
    }
}


fn draw_layer(layer: &Layer) {
    let mut output = String::new();
    for y in 0..layer.height {
        for x in 0..layer.width {
            output.push(get_char(layer.data[x + y * layer.width]));
        }
        output.push('\n');
    }
    println!("{}", output);
}

fn create_blank_image(width: usize, height: usize) -> Image {
    Image {
        width: width,
        height: height,
        layers: Vec::new(),
    }
}

fn part1(image: &Image) -> usize {
    let mut best_key = 1000;
    let mut best_value = 0;
    for layer in image.layers.iter() {
        let number_of_zeros = layer.count_char(0);
        if number_of_zeros < best_key {
            best_key = number_of_zeros;
            best_value = layer.count_char(1) * layer.count_char(2);
        }
    }
    
    best_value
}

fn part2(image: &Image) {
    println!("Part 2:\n");
    let layer = image.produce_layer();
    draw_layer(&layer);
}

pub fn execute() {
    let content = crate::input_files::read_content(&String::from("data/day08.txt"));
    let mut image = create_blank_image(25, 6);
    for c in content.trim().chars() {
        image.accept(c.to_digit(10).unwrap() as u8);
    }
    let result_of_part_1 = part1(&image);
    println!("Part 1: {}", result_of_part_1);
    part2(&image);
}
