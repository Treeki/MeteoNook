//import { Forecast, Hemisphere, MonthForecast } from '../js/model'
import { Hemisphere, Guesser, GuessData } from '../pkg'

function testSeed(seed: number) {
    //const month = new MonthForecast(Hemisphere.Northern, seed, 2020, 6)
    //const day = month.days[13]
    

    const guessData = GuessData.new(Hemisphere.Northern)

    const guessBase = seed & 0x7FFF0000
    const guesser = Guesser.new(guessBase, guessBase + 0xFFFF)
    guesser.work(guessData, 0x10000)
}

testSeed(1000)

