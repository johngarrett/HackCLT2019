var map, heatmap, pointArray;
function initMap() {
  pointArray = new google.maps.MVCArray();
  map = new google.maps.Map(document.getElementById('map'), {
    zoom: 13,
    center: {lat: 35.227085, lng: -80.843124},
    mapTypeId: 'roadmap'
  });
  heatmap = new google.maps.visualization.HeatmapLayer({
    data: getPoints(),
    map: map
  });
  heatmap.set('radius', 50);
}
function toggleHeatmap() {
  heatmap.setMap(heatmap.getMap() ? null : map);
}
function getPoints() {
  createPoint(35, -80);
  return pointArray;
}
function createPoint(x, y){
  pointArray.push(new google.maps.LatLng(x, y));
}