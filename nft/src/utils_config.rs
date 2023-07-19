pub fn get_monster_features() -> Vec<[[u16; 7]; 64]> {
    let monster_0: [[u16; 7]; 64] = [
        [250, 0, 0, 0, 0, 0, 0],
        [200, 50, 0, 0, 0, 0, 0],
        [150, 120, 0, 150, 0, 0, 0],
        [150, 150, 0, 140, 0, 0, 0],
        [50, 180, 0, 200, 0, 120, 0],
        [50, 200, 80, 180, 0, 0, 0],
        [50, 150, 0, 200, 0, 0, 100],
        [50, 250, 0, 260, 120, 0, 0],
        [25, 300, 140, 200, 0, 0, 0],
        [25, 300, 0, 280, 0, 200, 0],
        [200, 0, 0, 0, 0, 0, 0],
        [150, 0, 0, 0, 0, 50, 0],
        [150, 60, 0, 0, 0, 80, 0],
        [150, 0, 40, 0, 0, 100, 0],
        [100, 0, 0, 0, 0, 150, 80],
        [100, 80, 0, 0, 0, 180, 0],
        [50, 0, 100, 0, 50, 200, 0],
        [50, 0, 0, 120, 60, 220, 0],
        [25, 0, 0, 0, 80, 250, 100],
        [25, 0, 70, 0, 260, 100, 80],
        [150, 0, 0, 0, 0, 0, 0],
        [150, 50, 0, 0, 0, 0, 50],
        [150, 0, 80, 0, 0, 0, 60],
        [100, 0, 0, 120, 0, 0, 60],
        [100, 0, 0, 0, 150, 0, 70],
        [100, 50, 0, 90, 0, 0, 80],
        [50, 0, 100, 0, 0, 120, 100],
        [50, 70, 0, 110, 0, 0, 150],
        [50, 0, 90, 0, 120, 0, 180],
        [40, 0, 110, 0, 0, 130, 200],
        [40, 0, 0, 120, 160, 0, 260],
        [20, 70, 0, 90, 0, 100, 320],
        [150, 0, 50, 0, 0, 0, 0],
        [150, 0, 70, 30, 0, 0, 0],
        [150, 0, 80, 0, 0, 0, 40],
        [100, 0, 110, 0, 50, 0, 0],
        [100, 0, 130, 0, 0, 80, 0],
        [100, 0, 150, 80, 0, 0, 0],
        [50, 70, 160, 0, 80, 0, 0],
        [50, 0, 160, 90, 0, 100, 0],
        [50, 130, 170, 0, 0, 0, 100],
        [40, 0, 190, 110, 160, 0, 0],
        [30, 0, 200, 200, 0, 180, 0],
        [30, 190, 230, 0, 180, 0, 0],
        [300, 0, 0, 0, 0, 0, 0],
        [250, 0, 0, 0, 60, 0, 0],
        [200, 0, 70, 0, 70, 0, 0],
        [150, 0, 0, 90, 100, 50, 0],
        [100, 100, 0, 0, 140, 80, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
    ];

    let monster_1: [[u16; 7]; 64] = [
        [400, 0, 0, 0, 0, 0, 0],
        [200, 80, 0, 0, 0, 0, 0],
        [200, 120, 0, 50, 50, 0, 0],
        [100, 150, 0, 80, 0, 60, 0],
        [100, 200, 0, 120, 0, 0, 80],
        [700, 0, 0, 0, 0, 0, 0],
        [200, 100, 120, 0, 60, 0, 0],
        [100, 250, 260, 0, 0, 90, 0],
        [200, 0, 0, 0, 0, 0, 0],
        [150, 0, 0, 0, 0, 50, 0],
        [150, 60, 0, 0, 0, 80, 0],
        [150, 0, 40, 0, 0, 100, 0],
        [100, 0, 0, 70, 0, 150, 60],
        [100, 80, 0, 90, 0, 180, 0],
        [50, 0, 100, 0, 0, 200, 100],
        [50, 70, 0, 120, 0, 220, 0],
        [25, 0, 0, 0, 170, 250, 0],
        [25, 0, 70, 0, 260, 260, 0],
        [150, 0, 0, 0, 0, 0, 0],
        [150, 0, 50, 0, 0, 0, 0],
        [150, 0, 60, 50, 0, 0, 0],
        [100, 60, 80, 0, 0, 0, 0],
        [100, 0, 90, 0, 80, 0, 0],
        [100, 0, 110, 0, 0, 0, 60],
        [50, 0, 130, 80, 0, 60, 0],
        [50, 70, 150, 0, 90, 0, 0],
        [50, 0, 170, 0, 0, 110, 100],
        [40, 110, 180, 0, 110, 0, 0],
        [30, 0, 200, 100, 0, 120, 0],
        [30, 130, 210, 0, 140, 0, 0],
        [300, 0, 0, 0, 0, 0, 0],
        [250, 0, 0, 0, 60, 0, 0],
        [200, 0, 70, 0, 70, 0, 0],
        [150, 80, 0, 0, 100, 50, 0],
        [100, 0, 80, 0, 140, 0, 80],
        [700, 0, 0, 0, 0, 0, 0],
        [300, 0, 120, 0, 180, 0, 50],
        [200, 0, 0, 0, 0, 0, 0],
        [150, 0, 0, 40, 0, 0, 0],
        [150, 50, 0, 50, 0, 0, 0],
        [150, 0, 70, 60, 0, 0, 0],
        [150, 0, 0, 100, 0, 80, 0],
        [50, 100, 0, 120, 0, 0, 0],
        [50, 0, 110, 130, 0, 0, 0],
        [50, 0, 0, 150, 0, 0, 70],
        [25, 0, 0, 200, 160, 80, 0],
        [25, 190, 0, 220, 0, 110, 0],
        [600, 0, 0, 0, 0, 0, 0],
        [300, 0, 0, 0, 100, 90, 0],
        [100, 0, 110, 0, 140, 0, 40],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
    ];

    let monster_2: [[u16; 7]; 64] = [
        [400, 0, 0, 0, 0, 0, 0],
        [200, 80, 0, 0, 0, 0, 0],
        [200, 120, 0, 50, 50, 0, 0],
        [100, 150, 0, 80, 0, 60, 0],
        [100, 200, 0, 120, 0, 0, 80],
        [300, 0, 0, 0, 0, 0, 0],
        [200, 100, 120, 0, 60, 0, 0],
        [200, 0, 0, 0, 0, 0, 0],
        [200, 0, 0, 0, 0, 0, 0],
        [100, 250, 260, 0, 0, 90, 0],
        [400, 0, 0, 0, 0, 0, 0],
        [200, 0, 0, 0, 0, 50, 0],
        [150, 60, 0, 0, 0, 80, 0],
        [150, 0, 40, 0, 0, 100, 0],
        [100, 0, 0, 70, 0, 150, 60],
        [150, 0, 0, 0, 0, 0, 0],
        [150, 0, 50, 0, 0, 0, 0],
        [150, 0, 60, 50, 0, 0, 0],
        [100, 60, 80, 0, 0, 0, 0],
        [100, 0, 90, 0, 80, 0, 0],
        [100, 0, 110, 0, 0, 0, 60],
        [50, 0, 130, 80, 0, 60, 0],
        [50, 70, 150, 0, 90, 0, 0],
        [50, 0, 170, 0, 0, 110, 100],
        [40, 110, 180, 0, 110, 0, 0],
        [30, 0, 200, 100, 0, 120, 0],
        [30, 130, 210, 0, 140, 0, 0],
        [300, 0, 0, 0, 0, 0, 0],
        [250, 0, 0, 0, 60, 0, 0],
        [200, 0, 70, 0, 70, 0, 0],
        [150, 80, 0, 0, 100, 50, 0],
        [100, 0, 80, 0, 140, 0, 80],
        [500, 0, 0, 0, 0, 0, 0],
        [200, 0, 0, 0, 0, 0, 0],
        [100, 0, 0, 0, 0, 0, 0],
        [100, 0, 0, 0, 0, 0, 0],
        [100, 0, 120, 0, 180, 0, 50],
        [200, 0, 0, 0, 0, 0, 0],
        [150, 0, 0, 40, 0, 0, 0],
        [150, 50, 0, 50, 0, 0, 0],
        [150, 0, 70, 60, 0, 0, 0],
        [150, 0, 0, 100, 0, 80, 0],
        [50, 100, 0, 120, 0, 0, 0],
        [50, 0, 110, 130, 0, 0, 0],
        [50, 0, 0, 150, 0, 0, 70],
        [25, 0, 0, 200, 160, 80, 0],
        [25, 190, 0, 220, 0, 110, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
    ];

    let monster_3: [[u16; 7]; 64] = [
        [400, 0, 0, 0, 0, 0, 0],
        [200, 80, 0, 0, 0, 0, 0],
        [200, 120, 0, 50, 50, 0, 0],
        [100, 150, 0, 80, 0, 60, 0],
        [100, 200, 0, 120, 0, 0, 80],
        [400, 0, 0, 0, 0, 0, 0],
        [200, 0, 0, 0, 0, 50, 0],
        [150, 60, 0, 0, 0, 80, 0],
        [150, 0, 40, 0, 0, 100, 0],
        [100, 0, 0, 70, 0, 150, 60],
        [150, 0, 0, 0, 0, 0, 0],
        [150, 0, 50, 0, 0, 0, 0],
        [150, 0, 60, 50, 0, 0, 0],
        [100, 60, 80, 0, 0, 0, 0],
        [100, 0, 90, 0, 80, 0, 0],
        [100, 0, 110, 0, 0, 0, 60],
        [50, 0, 130, 80, 0, 60, 0],
        [50, 70, 150, 0, 90, 0, 0],
        [50, 0, 170, 0, 0, 110, 100],
        [40, 110, 180, 0, 110, 0, 0],
        [30, 0, 200, 100, 0, 120, 0],
        [30, 130, 210, 0, 140, 0, 0],
        [300, 0, 0, 0, 0, 0, 0],
        [250, 0, 0, 0, 60, 0, 0],
        [200, 0, 70, 0, 70, 0, 0],
        [150, 80, 0, 0, 100, 50, 0],
        [100, 0, 80, 0, 140, 0, 80],
        [500, 0, 0, 0, 0, 0, 0],
        [200, 0, 0, 0, 0, 0, 0],
        [100, 0, 0, 0, 0, 0, 0],
        [100, 0, 0, 0, 0, 0, 0],
        [100, 0, 120, 0, 180, 0, 50],
        [500, 0, 0, 0, 0, 0, 0],
        [300, 0, 0, 40, 0, 0, 0],
        [200, 50, 0, 50, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
    ];

    let monster_4: [[u16; 7]; 64] = [
        [400, 0, 0, 0, 0, 0, 0],
        [200, 80, 0, 0, 0, 0, 0],
        [200, 120, 0, 50, 50, 0, 0],
        [100, 150, 0, 80, 0, 60, 0],
        [100, 200, 0, 120, 0, 0, 80],
        [500, 0, 0, 0, 0, 0, 0],
        [300, 0, 0, 0, 0, 0, 0],
        [200, 0, 0, 0, 0, 0, 0],
        [400, 0, 0, 0, 0, 0, 0],
        [200, 0, 0, 0, 0, 50, 0],
        [150, 60, 0, 0, 0, 80, 0],
        [150, 0, 40, 0, 0, 100, 0],
        [100, 0, 0, 70, 0, 150, 60],
        [200, 0, 0, 0, 0, 0, 0],
        [200, 0, 50, 0, 0, 0, 0],
        [200, 0, 60, 50, 0, 0, 0],
        [200, 60, 80, 0, 0, 0, 0],
        [200, 0, 90, 0, 80, 0, 0],
        [300, 0, 0, 0, 0, 0, 0],
        [250, 0, 0, 0, 60, 0, 0],
        [200, 0, 70, 0, 70, 0, 0],
        [150, 80, 0, 0, 100, 50, 0],
        [100, 0, 80, 0, 140, 0, 80],
        [500, 0, 0, 0, 0, 0, 0],
        [200, 0, 0, 0, 0, 0, 0],
        [100, 0, 0, 0, 0, 0, 0],
        [100, 0, 0, 0, 0, 0, 0],
        [100, 0, 120, 0, 180, 0, 50],
        [300, 0, 0, 0, 0, 0, 0],
        [250, 0, 0, 40, 0, 0, 0],
        [200, 50, 0, 50, 0, 0, 0],
        [150, 0, 0, 0, 0, 0, 0],
        [100, 0, 0, 0, 0, 0, 0],
        [300, 0, 0, 0, 0, 0, 0],
        [250, 0, 0, 0, 0, 0, 0],
        [200, 0, 0, 0, 0, 0, 0],
        [150, 0, 0, 0, 0, 0, 0],
        [100, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
    ];

    let mut config = Vec::new();
    config.push(monster_0);
    config.push(monster_1);
    config.push(monster_2);
    config.push(monster_3);
    config.push(monster_4);

    return config;
}

pub fn get_monster_feature_by_index(index: usize) -> [[u16; 7]; 64] {
    let features = get_monster_features();
    return features[index];
}

pub fn handle_monster_feature_config(config: [[u16; 7]; 64]) -> Vec<Vec<[u16; 7]>> {
    let mut sum = 0;
    let mut features = Vec::new();
    let mut temp = Vec::new();
    for i in 0..config.len() {
        if config[i][0] == 0 {
            continue;
        };
        sum += config[i][0];
        if sum < 1000 {
            temp.push(config[i].clone());
        } else if sum == 1000 {
            features.push(temp.clone());
            temp = Vec::new();
            sum = 0;
        } else {
            panic!("Invalid config.");
        }
    }
    return features;
}

pub fn get_monster_male_basic_attrs() -> [[u32; 6]; 10] {
    let male_data: [[u32; 6]; 10] = [
        [100, 100, 120, 100, 120, 20],
        [100, 140, 110, 120, 150, 24],
        [150, 170, 160, 165, 165, 30],
        [816, 800, 832, 800, 712, 112],
        [100, 150, 120, 100, 120, 45],
        [630, 640, 620, 610, 650, 48],
        [700, 720, 690, 710, 700, 56],
        [830, 840, 820, 810, 850, 60],
        [940, 920, 890, 910, 940, 65],
        [1090, 1040, 1060, 1010, 1030, 80]
    ];

    return male_data;
}

pub fn get_monster_female_basic_attrs() -> [[u32; 6]; 10] {
    let female_data: [[u32; 6]; 10] = [
        [100, 120, 100, 120, 100, 20],
        [90, 140, 130, 130, 130, 24],
        [165, 165, 150, 170, 160, 30],
        [832, 800, 816, 800, 712, 112],
        [110, 160, 100, 120, 100, 45],
        [650, 610, 620, 630, 640, 48],
        [720, 690, 710, 700, 700, 56],
        [850, 810, 820, 830, 840, 60],
        [900, 890, 940, 960, 910, 65],
        [1050, 1080, 1040, 1010, 1050, 80]
    ];
    return female_data;
}

pub fn get_monster_basic_attrs_sum(race: u8) -> u32 {
    let male_data: [[u32; 6]; 10] = get_monster_male_basic_attrs();
    let race_data: [u32; 6] = male_data[race as usize];
    let mut sum: u32 = 0;
    for i in 0..6 {
        sum += race_data[i];
    }
    return sum;
}

pub fn calculate_battle_receive_game_token(win: u8, race: u8, level: u8) -> u64 {
    let mut attrs: f32 = get_monster_basic_attrs_sum(race) as f32;
    attrs *= 1.06_f32.powf(level as f32 - 1.0);
    attrs = attrs.powf(0.35);
    if win <= 0 {
        attrs /= 3.0;
    }
    let receive: u64 = (attrs * 1_000_000_000.0) as u64;
    return receive;
}

pub fn calculate_cure_spend_game_token(level: u8, cure: u8) -> u64 {
    let basic: u64;
    if level <= 10 {
        basic = 15;
    } else if level <= 20 {
        basic = 20;
    } else {
        basic = 25;
    }
    let spend: u64 = (basic * cure as u64 / 25) as u64;
    return spend * 1_000_000_000;
}

pub fn get_upgrade_base_lst(race: u8) -> u64 {
    if race == 0 {
        return 50;
    } else if race == 1 {
        return 58;
    } else if race == 2 {
        return 78;
    } else if race == 3 {
        return 390;
    } else if race == 4 {
        return 60;
    } else {
        return 100;
    }
}

pub fn calculate_upgrade_spend_game_token(race: u8, level: u8) -> u64 {
    let mut spend: f64 = get_upgrade_base_lst(race) as f64;
    if level <= 1 {
        return spend as u64 * 1_000_000_000;
    }
    for _i in 0..level - 1 {
        spend = (spend * 1.2).ceil();
    }
    return spend as u64 * 1_000_000_000;
}

pub fn get_breed_base_lst(race: u8) -> u64 {
    if race == 0 {
        return 1000;
    } else if race == 1 {
        return 1200;
    } else if race == 2 {
        return 1600;
    } else if race == 3 {
        return 3200;
    } else if race == 4 {
        return 1200;
    } else {
        return 2000;
    }
}

pub fn calculate_breed_spend_game_token(race: u8, breed01: u8, breed02: u8) -> u64 {
    let base = get_breed_base_lst(race);
    let price01 = breed01 as u64;
    let price02 = breed02 as u64;
    let spend = (price01 + price02 + 2) * base * (price01 + price02 + 1) / 2;
    return spend * 1_000_000_000;
}

pub fn calculate_synthesize_spend_game_token(race01: u8, race02: u8) -> u64 {
    let spend: u64;
    if race01 == 0 && race02 == 1 {
        spend = 50;
    } else if race01 == 1 && race02 == 0 {
        spend = 50;
    } else {
        spend = 0;
    }
    return spend * 1_000_000_000;
}
