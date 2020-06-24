import json

lines = [v.rstrip('\n') for v in open('translations.tsv', 'r').readlines() if len(v) > 0]
langs = lines.pop(0).split('\t')[1:]
data = {lang: {} for lang in langs}

for row in lines:
	row = row.split('\t')
	key = row.pop(0)
	if key == '': continue
	for lang,tstr in zip(langs,row):
		if key.startswith('lst'):
			# lists are special
			for i, bit in enumerate(tstr.split('%%')):
				data[lang]['%s%d' % (key, i)] = bit
		elif '%%' in tstr:
			# real funky HTML handling
			output_bits = []
			ul_flag = False
			for bit in tstr.split('%%'):
				if bit.startswith('-'):
					if not ul_flag:
						output_bits.append('<ul>')
						ul_flag = True
					output_bits.append('<li>')
					output_bits.append(bit[1:])
					output_bits.append('</li>')
				else:
					if ul_flag:
						output_bits.append('</ul>')
						ul_flag = False
					output_bits.append('<p>')
					output_bits.append(bit)
					output_bits.append('</p>')
			if ul_flag:
				output_bits.append('</ul>')
			data[lang][key] = ''.join(output_bits)
		else:
			# plain old string (mostly)
			data[lang][key] = tstr

with open('js/translations.js', 'w') as f:
	f.write('export default ')
	json.dump(data, f, indent=4)
