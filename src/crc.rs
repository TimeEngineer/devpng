//! # Crc

// Constants.
// const CRC_TABLE: [[u8; 4]; 256] = [
//     [0x00, 0x00, 0x00, 0x00],
//     [0x77, 0x07, 0x30, 0x96],
//     [0xee, 0x0e, 0x61, 0x2c],
//     [0x99, 0x09, 0x51, 0xba],
//     [0x07, 0x6d, 0xc4, 0x19],
//     [0x70, 0x6a, 0xf4, 0x8f],
//     [0xe9, 0x63, 0xa5, 0x35],
//     [0x9e, 0x64, 0x95, 0xa3],
//     [0x0e, 0xdb, 0x88, 0x32],
//     [0x79, 0xdc, 0xb8, 0xa4],
//     [0xe0, 0xd5, 0xe9, 0x1e],
//     [0x97, 0xd2, 0xd9, 0x88],
//     [0x09, 0xb6, 0x4c, 0x2b],
//     [0x7e, 0xb1, 0x7c, 0xbd],
//     [0xe7, 0xb8, 0x2d, 0x07],
//     [0x90, 0xbf, 0x1d, 0x91],
//     [0x1d, 0xb7, 0x10, 0x64],
//     [0x6a, 0xb0, 0x20, 0xf2],
//     [0xf3, 0xb9, 0x71, 0x48],
//     [0x84, 0xbe, 0x41, 0xde],
//     [0x1a, 0xda, 0xd4, 0x7d],
//     [0x6d, 0xdd, 0xe4, 0xeb],
//     [0xf4, 0xd4, 0xb5, 0x51],
//     [0x83, 0xd3, 0x85, 0xc7],
//     [0x13, 0x6c, 0x98, 0x56],
//     [0x64, 0x6b, 0xa8, 0xc0],
//     [0xfd, 0x62, 0xf9, 0x7a],
//     [0x8a, 0x65, 0xc9, 0xec],
//     [0x14, 0x01, 0x5c, 0x4f],
//     [0x63, 0x06, 0x6c, 0xd9],
//     [0xfa, 0x0f, 0x3d, 0x63],
//     [0x8d, 0x08, 0x0d, 0xf5],
//     [0x3b, 0x6e, 0x20, 0xc8],
//     [0x4c, 0x69, 0x10, 0x5e],
//     [0xd5, 0x60, 0x41, 0xe4],
//     [0xa2, 0x67, 0x71, 0x72],
//     [0x3c, 0x03, 0xe4, 0xd1],
//     [0x4b, 0x04, 0xd4, 0x47],
//     [0xd2, 0x0d, 0x85, 0xfd],
//     [0xa5, 0x0a, 0xb5, 0x6b],
//     [0x35, 0xb5, 0xa8, 0xfa],
//     [0x42, 0xb2, 0x98, 0x6c],
//     [0xdb, 0xbb, 0xc9, 0xd6],
//     [0xac, 0xbc, 0xf9, 0x40],
//     [0x32, 0xd8, 0x6c, 0xe3],
//     [0x45, 0xdf, 0x5c, 0x75],
//     [0xdc, 0xd6, 0x0d, 0xcf],
//     [0xab, 0xd1, 0x3d, 0x59],
//     [0x26, 0xd9, 0x30, 0xac],
//     [0x51, 0xde, 0x00, 0x3a],
//     [0xc8, 0xd7, 0x51, 0x80],
//     [0xbf, 0xd0, 0x61, 0x16],
//     [0x21, 0xb4, 0xf4, 0xb5],
//     [0x56, 0xb3, 0xc4, 0x23],
//     [0xcf, 0xba, 0x95, 0x99],
//     [0xb8, 0xbd, 0xa5, 0x0f],
//     [0x28, 0x02, 0xb8, 0x9e],
//     [0x5f, 0x05, 0x88, 0x08],
//     [0xc6, 0x0c, 0xd9, 0xb2],
//     [0xb1, 0x0b, 0xe9, 0x24],
//     [0x2f, 0x6f, 0x7c, 0x87],
//     [0x58, 0x68, 0x4c, 0x11],
//     [0xc1, 0x61, 0x1d, 0xab],
//     [0xb6, 0x66, 0x2d, 0x3d],
//     [0x76, 0xdc, 0x41, 0x90],
//     [0x01, 0xdb, 0x71, 0x06],
//     [0x98, 0xd2, 0x20, 0xbc],
//     [0xef, 0xd5, 0x10, 0x2a],
//     [0x71, 0xb1, 0x85, 0x89],
//     [0x06, 0xb6, 0xb5, 0x1f],
//     [0x9f, 0xbf, 0xe4, 0xa5],
//     [0xe8, 0xb8, 0xd4, 0x33],
//     [0x78, 0x07, 0xc9, 0xa2],
//     [0x0f, 0x00, 0xf9, 0x34],
//     [0x96, 0x09, 0xa8, 0x8e],
//     [0xe1, 0x0e, 0x98, 0x18],
//     [0x7f, 0x6a, 0x0d, 0xbb],
//     [0x08, 0x6d, 0x3d, 0x2d],
//     [0x91, 0x64, 0x6c, 0x97],
//     [0xe6, 0x63, 0x5c, 0x01],
//     [0x6b, 0x6b, 0x51, 0xf4],
//     [0x1c, 0x6c, 0x61, 0x62],
//     [0x85, 0x65, 0x30, 0xd8],
//     [0xf2, 0x62, 0x00, 0x4e],
//     [0x6c, 0x06, 0x95, 0xed],
//     [0x1b, 0x01, 0xa5, 0x7b],
//     [0x82, 0x08, 0xf4, 0xc1],
//     [0xf5, 0x0f, 0xc4, 0x57],
//     [0x65, 0xb0, 0xd9, 0xc6],
//     [0x12, 0xb7, 0xe9, 0x50],
//     [0x8b, 0xbe, 0xb8, 0xea],
//     [0xfc, 0xb9, 0x88, 0x7c],
//     [0x62, 0xdd, 0x1d, 0xdf],
//     [0x15, 0xda, 0x2d, 0x49],
//     [0x8c, 0xd3, 0x7c, 0xf3],
//     [0xfb, 0xd4, 0x4c, 0x65],
//     [0x4d, 0xb2, 0x61, 0x58],
//     [0x3a, 0xb5, 0x51, 0xce],
//     [0xa3, 0xbc, 0x00, 0x74],
//     [0xd4, 0xbb, 0x30, 0xe2],
//     [0x4a, 0xdf, 0xa5, 0x41],
//     [0x3d, 0xd8, 0x95, 0xd7],
//     [0xa4, 0xd1, 0xc4, 0x6d],
//     [0xd3, 0xd6, 0xf4, 0xfb],
//     [0x43, 0x69, 0xe9, 0x6a],
//     [0x34, 0x6e, 0xd9, 0xfc],
//     [0xad, 0x67, 0x88, 0x46],
//     [0xda, 0x60, 0xb8, 0xd0],
//     [0x44, 0x04, 0x2d, 0x73],
//     [0x33, 0x03, 0x1d, 0xe5],
//     [0xaa, 0x0a, 0x4c, 0x5f],
//     [0xdd, 0x0d, 0x7c, 0xc9],
//     [0x50, 0x05, 0x71, 0x3c],
//     [0x27, 0x02, 0x41, 0xaa],
//     [0xbe, 0x0b, 0x10, 0x10],
//     [0xc9, 0x0c, 0x20, 0x86],
//     [0x57, 0x68, 0xb5, 0x25],
//     [0x20, 0x6f, 0x85, 0xb3],
//     [0xb9, 0x66, 0xd4, 0x09],
//     [0xce, 0x61, 0xe4, 0x9f],
//     [0x5e, 0xde, 0xf9, 0x0e],
//     [0x29, 0xd9, 0xc9, 0x98],
//     [0xb0, 0xd0, 0x98, 0x22],
//     [0xc7, 0xd7, 0xa8, 0xb4],
//     [0x59, 0xb3, 0x3d, 0x17],
//     [0x2e, 0xb4, 0x0d, 0x81],
//     [0xb7, 0xbd, 0x5c, 0x3b],
//     [0xc0, 0xba, 0x6c, 0xad],
//     [0xed, 0xb8, 0x83, 0x20],
//     [0x9a, 0xbf, 0xb3, 0xb6],
//     [0x03, 0xb6, 0xe2, 0x0c],
//     [0x74, 0xb1, 0xd2, 0x9a],
//     [0xea, 0xd5, 0x47, 0x39],
//     [0x9d, 0xd2, 0x77, 0xaf],
//     [0x04, 0xdb, 0x26, 0x15],
//     [0x73, 0xdc, 0x16, 0x83],
//     [0xe3, 0x63, 0x0b, 0x12],
//     [0x94, 0x64, 0x3b, 0x84],
//     [0x0d, 0x6d, 0x6a, 0x3e],
//     [0x7a, 0x6a, 0x5a, 0xa8],
//     [0xe4, 0x0e, 0xcf, 0x0b],
//     [0x93, 0x09, 0xff, 0x9d],
//     [0x0a, 0x00, 0xae, 0x27],
//     [0x7d, 0x07, 0x9e, 0xb1],
//     [0xf0, 0x0f, 0x93, 0x44],
//     [0x87, 0x08, 0xa3, 0xd2],
//     [0x1e, 0x01, 0xf2, 0x68],
//     [0x69, 0x06, 0xc2, 0xfe],
//     [0xf7, 0x62, 0x57, 0x5d],
//     [0x80, 0x65, 0x67, 0xcb],
//     [0x19, 0x6c, 0x36, 0x71],
//     [0x6e, 0x6b, 0x06, 0xe7],
//     [0xfe, 0xd4, 0x1b, 0x76],
//     [0x89, 0xd3, 0x2b, 0xe0],
//     [0x10, 0xda, 0x7a, 0x5a],
//     [0x67, 0xdd, 0x4a, 0xcc],
//     [0xf9, 0xb9, 0xdf, 0x6f],
//     [0x8e, 0xbe, 0xef, 0xf9],
//     [0x17, 0xb7, 0xbe, 0x43],
//     [0x60, 0xb0, 0x8e, 0xd5],
//     [0xd6, 0xd6, 0xa3, 0xe8],
//     [0xa1, 0xd1, 0x93, 0x7e],
//     [0x38, 0xd8, 0xc2, 0xc4],
//     [0x4f, 0xdf, 0xf2, 0x52],
//     [0xd1, 0xbb, 0x67, 0xf1],
//     [0xa6, 0xbc, 0x57, 0x67],
//     [0x3f, 0xb5, 0x06, 0xdd],
//     [0x48, 0xb2, 0x36, 0x4b],
//     [0xd8, 0x0d, 0x2b, 0xda],
//     [0xaf, 0x0a, 0x1b, 0x4c],
//     [0x36, 0x03, 0x4a, 0xf6],
//     [0x41, 0x04, 0x7a, 0x60],
//     [0xdf, 0x60, 0xef, 0xc3],
//     [0xa8, 0x67, 0xdf, 0x55],
//     [0x31, 0x6e, 0x8e, 0xef],
//     [0x46, 0x69, 0xbe, 0x79],
//     [0xcb, 0x61, 0xb3, 0x8c],
//     [0xbc, 0x66, 0x83, 0x1a],
//     [0x25, 0x6f, 0xd2, 0xa0],
//     [0x52, 0x68, 0xe2, 0x36],
//     [0xcc, 0x0c, 0x77, 0x95],
//     [0xbb, 0x0b, 0x47, 0x03],
//     [0x22, 0x02, 0x16, 0xb9],
//     [0x55, 0x05, 0x26, 0x2f],
//     [0xc5, 0xba, 0x3b, 0xbe],
//     [0xb2, 0xbd, 0x0b, 0x28],
//     [0x2b, 0xb4, 0x5a, 0x92],
//     [0x5c, 0xb3, 0x6a, 0x04],
//     [0xc2, 0xd7, 0xff, 0xa7],
//     [0xb5, 0xd0, 0xcf, 0x31],
//     [0x2c, 0xd9, 0x9e, 0x8b],
//     [0x5b, 0xde, 0xae, 0x1d],
//     [0x9b, 0x64, 0xc2, 0xb0],
//     [0xec, 0x63, 0xf2, 0x26],
//     [0x75, 0x6a, 0xa3, 0x9c],
//     [0x02, 0x6d, 0x93, 0x0a],
//     [0x9c, 0x09, 0x06, 0xa9],
//     [0xeb, 0x0e, 0x36, 0x3f],
//     [0x72, 0x07, 0x67, 0x85],
//     [0x05, 0x00, 0x57, 0x13],
//     [0x95, 0xbf, 0x4a, 0x82],
//     [0xe2, 0xb8, 0x7a, 0x14],
//     [0x7b, 0xb1, 0x2b, 0xae],
//     [0x0c, 0xb6, 0x1b, 0x38],
//     [0x92, 0xd2, 0x8e, 0x9b],
//     [0xe5, 0xd5, 0xbe, 0x0d],
//     [0x7c, 0xdc, 0xef, 0xb7],
//     [0x0b, 0xdb, 0xdf, 0x21],
//     [0x86, 0xd3, 0xd2, 0xd4],
//     [0xf1, 0xd4, 0xe2, 0x42],
//     [0x68, 0xdd, 0xb3, 0xf8],
//     [0x1f, 0xda, 0x83, 0x6e],
//     [0x81, 0xbe, 0x16, 0xcd],
//     [0xf6, 0xb9, 0x26, 0x5b],
//     [0x6f, 0xb0, 0x77, 0xe1],
//     [0x18, 0xb7, 0x47, 0x77],
//     [0x88, 0x08, 0x5a, 0xe6],
//     [0xff, 0x0f, 0x6a, 0x70],
//     [0x66, 0x06, 0x3b, 0xca],
//     [0x11, 0x01, 0x0b, 0x5c],
//     [0x8f, 0x65, 0x9e, 0xff],
//     [0xf8, 0x62, 0xae, 0x69],
//     [0x61, 0x6b, 0xff, 0xd3],
//     [0x16, 0x6c, 0xcf, 0x45],
//     [0xa0, 0x0a, 0xe2, 0x78],
//     [0xd7, 0x0d, 0xd2, 0xee],
//     [0x4e, 0x04, 0x83, 0x54],
//     [0x39, 0x03, 0xb3, 0xc2],
//     [0xa7, 0x67, 0x26, 0x61],
//     [0xd0, 0x60, 0x16, 0xf7],
//     [0x49, 0x69, 0x47, 0x4d],
//     [0x3e, 0x6e, 0x77, 0xdb],
//     [0xae, 0xd1, 0x6a, 0x4a],
//     [0xd9, 0xd6, 0x5a, 0xdc],
//     [0x40, 0xdf, 0x0b, 0x66],
//     [0x37, 0xd8, 0x3b, 0xf0],
//     [0xa9, 0xbc, 0xae, 0x53],
//     [0xde, 0xbb, 0x9e, 0xc5],
//     [0x47, 0xb2, 0xcf, 0x7f],
//     [0x30, 0xb5, 0xff, 0xe9],
//     [0xbd, 0xbd, 0xf2, 0x1c],
//     [0xca, 0xba, 0xc2, 0x8a],
//     [0x53, 0xb3, 0x93, 0x30],
//     [0x24, 0xb4, 0xa3, 0xa6],
//     [0xba, 0xd0, 0x36, 0x05],
//     [0xcd, 0xd7, 0x06, 0x93],
//     [0x54, 0xde, 0x57, 0x29],
//     [0x23, 0xd9, 0x67, 0xbf],
//     [0xb3, 0x66, 0x7a, 0x2e],
//     [0xc4, 0x61, 0x4a, 0xb8],
//     [0x5d, 0x68, 0x1b, 0x02],
//     [0x2a, 0x6f, 0x2b, 0x94],
//     [0xb4, 0x0b, 0xbe, 0x37],
//     [0xc3, 0x0c, 0x8e, 0xa1],
//     [0x5a, 0x05, 0xdf, 0x1b],
//     [0x2d, 0x02, 0xef, 0x8d],
// ];
// // Structures.
// #[derive(Debug, Clone, Copy)]
// pub struct Crc([u8; 4]);
// // Implementations.
// impl Crc {
//     pub fn new() -> Self {
//         Self([0xFF, 0xFF, 0xFF, 0xFF])
//     }
//     pub fn update(&mut self, buf: &[u8]) {
//         for byte in buf {
//             let a = (self.0[3] ^ *byte) as usize;
//             let b = [0x00, self.0[0], self.0[1], self.0[2]];
//             self.0 = [
//                 CRC_TABLE[a][0] ^ b[0],
//                 CRC_TABLE[a][1] ^ b[1],
//                 CRC_TABLE[a][2] ^ b[2],
//                 CRC_TABLE[a][3] ^ b[3],
//             ];
//         }
//     }
//     pub fn checksum(&self) -> [u8; 4] {
//         [
//             self.0[0] ^ 0xFF,
//             self.0[1] ^ 0xFF,
//             self.0[2] ^ 0xFF,
//             self.0[3] ^ 0xFF,
//         ]
//     }
// }
