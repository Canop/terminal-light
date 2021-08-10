use crate::Rgb;

/// RGB values of ANSI color codes
pub(crate) static ANSI_TO_RGB: &[Rgb] = &[
    // 8 "standard" colors
    // The special thing about those ones is that there's not standard: they may
    // change with the configuration
    Rgb { r: 0, g: 0, b: 0 },
    Rgb { r: 128, g: 128, b: 128 },
    Rgb { r: 0, g: 0, b: 0 },
    Rgb { r: 128, g: 128, b: 128 },
    Rgb { r: 0, g: 0, b: 0 },
    Rgb { r: 128, g: 128, b: 128 },
    Rgb { r: 0, g: 0, b: 0 },
    Rgb { r: 192, g: 192, b: 192 },

    // 8 "high intensity" colors
    // May also be changed by user configuration
    Rgb { r: 128, g: 128, b: 128 },
    Rgb { r: 255, g: 255, b: 255 },
    Rgb { r: 0, g: 0, b: 0 },
    Rgb { r: 255, g: 255, b: 255 },
    Rgb { r: 0, g: 0, b: 0 },
    Rgb { r: 255, g: 255, b: 255 },
    Rgb { r: 0, g: 0, b: 0 },
    Rgb { r: 255, g: 255, b: 255 },

    // Medium codes
    // Those ones are supposed to be quite "standard"
    Rgb { r:0, g:0, b:0 },	    // ANSI 16
    Rgb { r:0, g:0, b:95 },	    // ANSI 17
    Rgb { r:0, g:0, b:135 },	    // ANSI 18
    Rgb { r:0, g:0, b:175 },	    // ANSI 19
    Rgb { r:0, g:0, b:215 },	    // ANSI 20
    Rgb { r:0, g:0, b:255 },	    // ANSI 21
    Rgb { r:0, g:95, b:0 },	    // ANSI 22
    Rgb { r:0, g:95, b:95 },	    // ANSI 23
    Rgb { r:0, g:95, b:135 },	    // ANSI 24
    Rgb { r:0, g:95, b:175 },	    // ANSI 25
    Rgb { r:0, g:95, b:215 },	    // ANSI 26
    Rgb { r:0, g:95, b:255 },	    // ANSI 27
    Rgb { r:0, g:135, b:0 },	    // ANSI 28
    Rgb { r:0, g:135, b:95 },	    // ANSI 29
    Rgb { r:0, g:135, b:135 },	    // ANSI 30
    Rgb { r:0, g:135, b:175 },	    // ANSI 31
    Rgb { r:0, g:135, b:215 },	    // ANSI 32
    Rgb { r:0, g:135, b:255 },	    // ANSI 33
    Rgb { r:0, g:175, b:0 },	    // ANSI 34
    Rgb { r:0, g:175, b:95 },	    // ANSI 35
    Rgb { r:0, g:175, b:135 },	    // ANSI 36
    Rgb { r:0, g:175, b:175 },	    // ANSI 37
    Rgb { r:0, g:175, b:215 },	    // ANSI 38
    Rgb { r:0, g:175, b:255 },	    // ANSI 39
    Rgb { r:0, g:215, b:0 },	    // ANSI 40
    Rgb { r:0, g:215, b:95 },	    // ANSI 41
    Rgb { r:0, g:215, b:135 },	    // ANSI 42
    Rgb { r:0, g:215, b:175 },	    // ANSI 43
    Rgb { r:0, g:215, b:215 },	    // ANSI 44
    Rgb { r:0, g:215, b:255 },	    // ANSI 45
    Rgb { r:0, g:255, b:0 },	    // ANSI 46
    Rgb { r:0, g:255, b:95 },	    // ANSI 47
    Rgb { r:0, g:255, b:135 },	    // ANSI 48
    Rgb { r:0, g:255, b:175 },	    // ANSI 49
    Rgb { r:0, g:255, b:215 },	    // ANSI 50
    Rgb { r:0, g:255, b:255 },	    // ANSI 51
    Rgb { r:95, g:0, b:0 },	    // ANSI 52
    Rgb { r:95, g:0, b:95 },	    // ANSI 53
    Rgb { r:95, g:0, b:135 },	    // ANSI 54
    Rgb { r:95, g:0, b:175 },	    // ANSI 55
    Rgb { r:95, g:0, b:215 },	    // ANSI 56
    Rgb { r:95, g:0, b:255 },	    // ANSI 57
    Rgb { r:95, g:95, b:0 },	    // ANSI 58
    Rgb { r:95, g:95, b:95 },	    // ANSI 59
    Rgb { r:95, g:95, b:135 },	    // ANSI 60
    Rgb { r:95, g:95, b:175 },	    // ANSI 61
    Rgb { r:95, g:95, b:215 },	    // ANSI 62
    Rgb { r:95, g:95, b:255 },	    // ANSI 63
    Rgb { r:95, g:135, b:0 },	    // ANSI 64
    Rgb { r:95, g:135, b:95 },	    // ANSI 65
    Rgb { r:95, g:135, b:135 },	    // ANSI 66
    Rgb { r:95, g:135, b:175 },	    // ANSI 67
    Rgb { r:95, g:135, b:215 },	    // ANSI 68
    Rgb { r:95, g:135, b:255 },	    // ANSI 69
    Rgb { r:95, g:175, b:0 },	    // ANSI 70
    Rgb { r:95, g:175, b:95 },	    // ANSI 71
    Rgb { r:95, g:175, b:135 },	    // ANSI 72
    Rgb { r:95, g:175, b:175 },	    // ANSI 73
    Rgb { r:95, g:175, b:215 },	    // ANSI 74
    Rgb { r:95, g:175, b:255 },	    // ANSI 75
    Rgb { r:95, g:215, b:0 },	    // ANSI 76
    Rgb { r:95, g:215, b:95 },	    // ANSI 77
    Rgb { r:95, g:215, b:135 },	    // ANSI 78
    Rgb { r:95, g:215, b:175 },	    // ANSI 79
    Rgb { r:95, g:215, b:215 },	    // ANSI 80
    Rgb { r:95, g:215, b:255 },	    // ANSI 81
    Rgb { r:95, g:255, b:0 },	    // ANSI 82
    Rgb { r:95, g:255, b:95 },	    // ANSI 83
    Rgb { r:95, g:255, b:135 },	    // ANSI 84
    Rgb { r:95, g:255, b:175 },	    // ANSI 85
    Rgb { r:95, g:255, b:215 },	    // ANSI 86
    Rgb { r:95, g:255, b:255 },	    // ANSI 87
    Rgb { r:135, g:0, b:0 },	    // ANSI 88
    Rgb { r:135, g:0, b:95 },	    // ANSI 89
    Rgb { r:135, g:0, b:135 },	    // ANSI 90
    Rgb { r:135, g:0, b:175 },	    // ANSI 91
    Rgb { r:135, g:0, b:215 },	    // ANSI 92
    Rgb { r:135, g:0, b:255 },	    // ANSI 93
    Rgb { r:135, g:95, b:0 },	    // ANSI 94
    Rgb { r:135, g:95, b:95 },	    // ANSI 95
    Rgb { r:135, g:95, b:135 },	    // ANSI 96
    Rgb { r:135, g:95, b:175 },	    // ANSI 97
    Rgb { r:135, g:95, b:215 },	    // ANSI 98
    Rgb { r:135, g:95, b:255 },	    // ANSI 99
    Rgb { r:135, g:135, b:0 },	    // ANSI 100
    Rgb { r:135, g:135, b:95 },	    // ANSI 101
    Rgb { r:135, g:135, b:135 },    // ANSI 102
    Rgb { r:135, g:135, b:175 },    // ANSI 103
    Rgb { r:135, g:135, b:215 },    // ANSI 104
    Rgb { r:135, g:135, b:255 },    // ANSI 105
    Rgb { r:135, g:175, b:0 },	    // ANSI 106
    Rgb { r:135, g:175, b:95 },	    // ANSI 107
    Rgb { r:135, g:175, b:135 },    // ANSI 108
    Rgb { r:135, g:175, b:175 },    // ANSI 109
    Rgb { r:135, g:175, b:215 },    // ANSI 110
    Rgb { r:135, g:175, b:255 },    // ANSI 111
    Rgb { r:135, g:215, b:0 },	    // ANSI 112
    Rgb { r:135, g:215, b:95 },	    // ANSI 113
    Rgb { r:135, g:215, b:135 },    // ANSI 114
    Rgb { r:135, g:215, b:175 },    // ANSI 115
    Rgb { r:135, g:215, b:215 },    // ANSI 116
    Rgb { r:135, g:215, b:255 },    // ANSI 117
    Rgb { r:135, g:255, b:0 },	    // ANSI 118
    Rgb { r:135, g:255, b:95 },	    // ANSI 119
    Rgb { r:135, g:255, b:135 },    // ANSI 120
    Rgb { r:135, g:255, b:175 },    // ANSI 121
    Rgb { r:135, g:255, b:215 },    // ANSI 122
    Rgb { r:135, g:255, b:255 },    // ANSI 123
    Rgb { r:175, g:0, b:0 },	    // ANSI 124
    Rgb { r:175, g:0, b:95 },	    // ANSI 125
    Rgb { r:175, g:0, b:135 },	    // ANSI 126
    Rgb { r:175, g:0, b:175 },	    // ANSI 127
    Rgb { r:175, g:0, b:215 },	    // ANSI 128
    Rgb { r:175, g:0, b:255 },	    // ANSI 129
    Rgb { r:175, g:95, b:0 },	    // ANSI 130
    Rgb { r:175, g:95, b:95 },	    // ANSI 131
    Rgb { r:175, g:95, b:135 },	    // ANSI 132
    Rgb { r:175, g:95, b:175 },	    // ANSI 133
    Rgb { r:175, g:95, b:215 },	    // ANSI 134
    Rgb { r:175, g:95, b:255 },	    // ANSI 135
    Rgb { r:175, g:135, b:0 },	    // ANSI 136
    Rgb { r:175, g:135, b:95 },	    // ANSI 137
    Rgb { r:175, g:135, b:135 },    // ANSI 138
    Rgb { r:175, g:135, b:175 },    // ANSI 139
    Rgb { r:175, g:135, b:215 },    // ANSI 140
    Rgb { r:175, g:135, b:255 },    // ANSI 141
    Rgb { r:175, g:175, b:0 },	    // ANSI 142
    Rgb { r:175, g:175, b:95 },	    // ANSI 143
    Rgb { r:175, g:175, b:135 },    // ANSI 144
    Rgb { r:175, g:175, b:175 },    // ANSI 145
    Rgb { r:175, g:175, b:215 },    // ANSI 146
    Rgb { r:175, g:175, b:255 },    // ANSI 147
    Rgb { r:175, g:215, b:0 },	    // ANSI 148
    Rgb { r:175, g:215, b:95 },	    // ANSI 149
    Rgb { r:175, g:215, b:135 },    // ANSI 150
    Rgb { r:175, g:215, b:175 },    // ANSI 151
    Rgb { r:175, g:215, b:215 },    // ANSI 152
    Rgb { r:175, g:215, b:255 },    // ANSI 153
    Rgb { r:175, g:255, b:0 },	    // ANSI 154
    Rgb { r:175, g:255, b:95 },	    // ANSI 155
    Rgb { r:175, g:255, b:135 },    // ANSI 156
    Rgb { r:175, g:255, b:175 },    // ANSI 157
    Rgb { r:175, g:255, b:215 },    // ANSI 158
    Rgb { r:175, g:255, b:255 },    // ANSI 159
    Rgb { r:215, g:0, b:0 },	    // ANSI 160
    Rgb { r:215, g:0, b:95 },	    // ANSI 161
    Rgb { r:215, g:0, b:135 },	    // ANSI 162
    Rgb { r:215, g:0, b:175 },	    // ANSI 163
    Rgb { r:215, g:0, b:215 },	    // ANSI 164
    Rgb { r:215, g:0, b:255 },	    // ANSI 165
    Rgb { r:215, g:95, b:0 },	    // ANSI 166
    Rgb { r:215, g:95, b:95 },	    // ANSI 167
    Rgb { r:215, g:95, b:135 },	    // ANSI 168
    Rgb { r:215, g:95, b:175 },	    // ANSI 169
    Rgb { r:215, g:95, b:215 },	    // ANSI 170
    Rgb { r:215, g:95, b:255 },	    // ANSI 171
    Rgb { r:215, g:135, b:0 },	    // ANSI 172
    Rgb { r:215, g:135, b:95 },	    // ANSI 173
    Rgb { r:215, g:135, b:135 },    // ANSI 174
    Rgb { r:215, g:135, b:175 },    // ANSI 175
    Rgb { r:215, g:135, b:215 },    // ANSI 176
    Rgb { r:215, g:135, b:255 },    // ANSI 177
    Rgb { r:215, g:175, b:0 },	    // ANSI 178
    Rgb { r:215, g:175, b:95 },	    // ANSI 179
    Rgb { r:215, g:175, b:135 },    // ANSI 180
    Rgb { r:215, g:175, b:175 },    // ANSI 181
    Rgb { r:215, g:175, b:215 },    // ANSI 182
    Rgb { r:215, g:175, b:255 },    // ANSI 183
    Rgb { r:215, g:215, b:0 },	    // ANSI 184
    Rgb { r:215, g:215, b:95 },	    // ANSI 185
    Rgb { r:215, g:215, b:135 },    // ANSI 186
    Rgb { r:215, g:215, b:175 },    // ANSI 187
    Rgb { r:215, g:215, b:215 },    // ANSI 188
    Rgb { r:215, g:215, b:255 },    // ANSI 189
    Rgb { r:215, g:255, b:0 },	    // ANSI 190
    Rgb { r:215, g:255, b:95 },	    // ANSI 191
    Rgb { r:215, g:255, b:135 },    // ANSI 192
    Rgb { r:215, g:255, b:175 },    // ANSI 193
    Rgb { r:215, g:255, b:215 },    // ANSI 194
    Rgb { r:215, g:255, b:255 },    // ANSI 195
    Rgb { r:255, g:0, b:0 },	    // ANSI 196
    Rgb { r:255, g:0, b:95 },	    // ANSI 197
    Rgb { r:255, g:0, b:135 },	    // ANSI 198
    Rgb { r:255, g:0, b:175 },	    // ANSI 199
    Rgb { r:255, g:0, b:215 },	    // ANSI 200
    Rgb { r:255, g:0, b:255 },	    // ANSI 201
    Rgb { r:255, g:95, b:0 },	    // ANSI 202
    Rgb { r:255, g:95, b:95 },	    // ANSI 203
    Rgb { r:255, g:95, b:135 },	    // ANSI 204
    Rgb { r:255, g:95, b:175 },	    // ANSI 205
    Rgb { r:255, g:95, b:215 },	    // ANSI 206
    Rgb { r:255, g:95, b:255 },	    // ANSI 207
    Rgb { r:255, g:135, b:0 },	    // ANSI 208
    Rgb { r:255, g:135, b:95 },	    // ANSI 209
    Rgb { r:255, g:135, b:135 },    // ANSI 210
    Rgb { r:255, g:135, b:175 },    // ANSI 211
    Rgb { r:255, g:135, b:215 },    // ANSI 212
    Rgb { r:255, g:135, b:255 },    // ANSI 213
    Rgb { r:255, g:175, b:0 },	    // ANSI 214
    Rgb { r:255, g:175, b:95 },	    // ANSI 215
    Rgb { r:255, g:175, b:135 },    // ANSI 216
    Rgb { r:255, g:175, b:175 },    // ANSI 217
    Rgb { r:255, g:175, b:215 },    // ANSI 218
    Rgb { r:255, g:175, b:255 },    // ANSI 219
    Rgb { r:255, g:215, b:0 },	    // ANSI 220
    Rgb { r:255, g:215, b:95 },	    // ANSI 221
    Rgb { r:255, g:215, b:135 },    // ANSI 222
    Rgb { r:255, g:215, b:175 },    // ANSI 223
    Rgb { r:255, g:215, b:215 },    // ANSI 224
    Rgb { r:255, g:215, b:255 },    // ANSI 225
    Rgb { r:255, g:255, b:0 },	    // ANSI 226
    Rgb { r:255, g:255, b:95 },	    // ANSI 227
    Rgb { r:255, g:255, b:135 },    // ANSI 228
    Rgb { r:255, g:255, b:175 },    // ANSI 229
    Rgb { r:255, g:255, b:215 },    // ANSI 230
    Rgb { r:255, g:255, b:255 },    // ANSI 231

    // 24 gray levels
    Rgb { r:8, g:8, b:8 },		// ANSI 232 : gray 0
    Rgb { r:18, g:18, b:18 },		// ANSI 233 : gray 1
    Rgb { r:28, g:28, b:28 },		// ANSI 234 : gray 2
    Rgb { r:38, g:38, b:38 },		// ANSI 235 : gray 3
    Rgb { r:48, g:48, b:48 },		// ANSI 236 : gray 4
    Rgb { r:58, g:58, b:58 },		// ANSI 237 : gray 5
    Rgb { r:68, g:68, b:68 },		// ANSI 238 : gray 6
    Rgb { r:78, g:78, b:78 },		// ANSI 239 : gray 7
    Rgb { r:88, g:88, b:88 },		// ANSI 240 : gray 8
    Rgb { r:98, g:98, b:98 },		// ANSI 241 : gray 9
    Rgb { r:108, g:108, b:108 },	// ANSI 242 : gray 10
    Rgb { r:118, g:118, b:118 },	// ANSI 243 : gray 11
    Rgb { r:128, g:128, b:128 },	// ANSI 244 : gray 12
    Rgb { r:138, g:138, b:138 },	// ANSI 245 : gray 13
    Rgb { r:148, g:148, b:148 },	// ANSI 246 : gray 14
    Rgb { r:158, g:158, b:158 },	// ANSI 247 : gray 15
    Rgb { r:168, g:168, b:168 },	// ANSI 248 : gray 16
    Rgb { r:178, g:178, b:178 },	// ANSI 249 : gray 17
    Rgb { r:188, g:188, b:188 },	// ANSI 250 : gray 18
    Rgb { r:198, g:198, b:198 },	// ANSI 251 : gray 19
    Rgb { r:208, g:208, b:208 },	// ANSI 252 : gray 20
    Rgb { r:218, g:218, b:218 },	// ANSI 253 : gray 21
    Rgb { r:228, g:228, b:228 },	// ANSI 254 : gray 22
    Rgb { r:238, g:238, b:238 },	// ANSI 255 : gray 23
];

