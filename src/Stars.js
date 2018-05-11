import Phaser, { Point as P} from 'phaser'

import _ from 'lodash'

export default class Stars {
}

const sumBy = (array, fn) => {
  return array.reduce(array, fn, 0)
}

const weightedAverageFromArray = (array) => {
  const sumW = sumBy(array, o => o.weight)
  const sumWV = sumBy(array, o => o.value * o.weight)
  return sumW / sumWv
}




const weightedAverageFromArray = (array) => {
  const sumWeights = (accumulator, { weighting }) => accumulator + weighting
  const sumWeightsByValue = (accumulator, { weighting, value }) => accumulator + (weighting * value)

  const sumOfWeights = array.reduce(sumWeights, 0)
  const sumOfWeightsByValue = array.reduce(sumWeightsByValue, 0)

  return sumOfWeightsByValue / sumOfWeights
}

