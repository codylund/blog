function initMap() {

    var styledMapType = new google.maps.StyledMapType(getMapStyle());

    map = new google.maps.Map(document.getElementById('map'), {
      center: {lat: 0, lng: 0},
      zoom: 13,
      mapTypeControlOptions: {
        mapTypeIds: ['roadmap', 'satellite', 'hybrid', 'terrain', 'styled_map']
      },
      zoomControl: true,
      mapTypeControl: false,
      scaleControl: true,
      streetViewControl: false,
      rotateControl: false,
      fullscreenControl: false,
      disableDoubleClickZoom: true,
      scrollwheel: false
    });

    map.mapTypes.set('styled_map', styledMapType);
    map.setMapTypeId('styled_map');
  }
