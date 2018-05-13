import $ from 'jquery'

function makePlayerPanel (players) {
  let list = $('<ul />')

  players.forEach(({name, score}) => {
    const li = $('<li />')

    li.append($('<span />').html(name))
    li.append($('<span />').html(score.toString()))

    list.append(li)
  })

  return list
}

export function updatePlayersPanel (players) {
  let list = makePlayerPanel(players)
  let names = $('#people')
  names.empty().append(list)
}
