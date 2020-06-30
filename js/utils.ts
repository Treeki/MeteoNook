export function makeTime(hours: number, minutes: number, seconds?: number): Date {
	// originally I was using 0,0,0 in the date field
	// but this doesn't play nicely with some timezones, like Portugal
	return new Date(2020, 6, 30, hours, minutes, seconds || 0)
}

export function readStorageObject<T>(key: string, validCallback?: (value: T) => boolean): T|null {
	try {
		const blob = window.localStorage.getItem(key)
		if (blob !== null) {
			const data = JSON.parse(blob)
			if (typeof data === 'object' && (validCallback === undefined || validCallback(data))) {
				return data
			}
		}
	} catch (ex) {
		console.log('error decoding ' + key)
		console.log(ex)
	}
	return null
}

export function readStorage<T>(key: string, decodeCallback: (blob: string) => T|null): T|null {
	try {
		const blob = window.localStorage.getItem(key)
		if (blob !== null) {
			return decodeCallback(blob)
		}
	} catch (ex) {
		console.log('error decoding ' + key)
		console.log(ex)
    }
    return null
}

export function writeStorageObject<T>(key: string, obj: T) {
	try {
		window.localStorage.setItem(key, JSON.stringify(obj))
	} catch (ex) {
		console.log('failed to save ' + key)
	}
}

export function writeStorage(key: string, blob: string) {
	try {
		window.localStorage.setItem(key, blob)
	} catch (ex) {
		console.log('failed to save ' + key)
	}
}

