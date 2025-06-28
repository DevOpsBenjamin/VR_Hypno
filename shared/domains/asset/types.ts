export interface AssetInfo {
    name: string;
    tags?: string[];
}
  
// Pour l'usage UI/backend :
export interface Asset {
  uid: string;
  info: AssetInfo;
} 