/**
 * @param {number[][]} rooms
 * @return {boolean}
 */

let seenRooms = {};

var canVisitAllRooms = function(rooms) {
    seenRooms = {};
    visitRoom(rooms, 0);
    return rooms.length == Object.keys(seenRooms).length;
};

var visitRoom = function(rooms, currentRoom) {
    if (seenRooms.hasOwnProperty(currentRoom)) return;
    seenRooms[currentRoom] = true;
    rooms[currentRoom].forEach((r) => visitRoom(rooms, r));
}
