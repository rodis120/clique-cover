#include <iostream>
#include <set>
#include <random>
#include <fstream>
#include <vector>
#include <string>
#include <cmath>
#include <numeric>
#include <algorithm>
#define MAX 999999
using namespace std;
struct Vertex
{
    double x, y;
    int nr,part;
};
string char_to_bin(char a)
{
    int b=int(a)-48;
    string w="";
    while(b>0)
    {
        w=char(b%2+48)+w;
        b=b/2;
    }
    while(w.size()<6)
    {
        w="0"+w;
    }
    return w;
}
std::string toLittleEndianBase64(string input) {
    while(input.size()%4!=0)
    {
        input=input+"0";
    }
    string binary="";
    string w="";
    for(int i=0;i<input.size();i+=4)
    {
        for(int j=i;j<=i+3;j++)
        {
            binary=binary+char_to_bin(input[j]);
        }
        reverse(binary.begin(),binary.end());
        for(int j=0;j<binary.size();j+=6)
        {
            string p=binary.substr(j,6);
            if(p=="000000")
                {
                    w=w+"A";

                }
                if(p=="000001")
                {
                    w=w+"B";

                }
                if(p=="000010")
                {
                    w=w+"C";

                }
                if(p=="000011")
                {
                    w=w+"D";

                }
                if(p=="000100")
                {
                    w=w+"E";

                }
                if(p=="000101")
                {
                    w=w+"F";

                }
                if(p=="000110")
                {
                    w=w+"G";

                }
                if(p=="000111")
                {
                    w=w+"H";

                }
                if(p=="001000")
                {
                    w=w+"I";

                }
                if(p=="001001")
                {
                    w=w+"J";

                }
                if(p== "001010")
                {
                    w=w+"K";

                }
                if(p=="001011")
                {
                    w=w+"L";

                }
                if(p=="001100")
                {
                    w=w+"M";

                }
                if(p=="001101")
                {
                    w=w+"N";

                }
                if(p=="001110")
                {
                    w=w+"O";

                }
                if(p=="001111")
                {
                    w=w+"P";

                }
                if(p=="010000")
                {
                    w=w+"Q";

                }
                if(p=="010001")
                {
                    w=w+"R";

                }
                if(p=="010010")
                {
                    w=w+"S";

                }
                if(p=="010011")
                {
                    w=w+"T";

                }
                if(p=="010100")
                {
                    w=w+"U";

                }
                if(p=="010101")
                {
                    w=w+"V";

                }
                if(p=="010110")
                {
                    w=w+"W";

                }
               if(p=="010111")
                {
                    w=w+"X";

                }
                if(p=="011000")
                {
                    w=w+"Y";

                }
                if(p=="011001")
                {
                    w=w+"Z";

                }
                if(p=="011010")
                {
                    w=w+"a";

                }
                if(p=="011011")
                {
                    w=w+"b";

                }
                if(p=="011100")
                {
                    w=w+"c";

                }
                if(p=="011101")
                {
                    w=w+"d";

                }
                if(p=="011110")
                {
                    w=w+"e";

                }
                if(p=="011111")
                {
                    w=w+"f";

                }
                if(p=="100000")
                {
                    w=w+"g";

                }
                if(p=="100001")
                {
                    w=w+"h";

                }
                if(p=="100010")
                {
                    w=w+"i";

                }
                if(p=="100011")
                {
                    w=w+"j";

                }
                if(p=="100100")
                {
                    w=w+"k";

                }
                if(p=="100101")
                {
                    w=w+"l";

                }
                if(p=="100110")
                {
                    w=w+"m";

                }
                if(p=="100111")
                {
                    w=w+"n";

                }
                if(p=="101000")
                {
                    w=w+"o";

                }
                if(p=="101001")
                {
                    w=w+"p";

                }
                if(p=="101010")
                {
                    w=w+"q";

                }
                if(p=="101011")
                {
                    w=w+"r";

                }
                if(p=="101100")
                {
                    w=w+"s";

                }
                if(p=="101101")
                {
                    w=w+"t";

                }
                if(p=="101110")
                {
                    w=w+"u";

                }
                if(p=="101111")
                {
                    w=w+"v";

                }
                if(p=="110000")
                {
                    w=w+"w";

                }
                if(p=="110001")
                {
                    w=w+"x";

                }
                if(p=="110010")
                {
                    w=w+"y";

                }
                if(p=="110011")
                {
                    w=w+"z";

                }
                if(p=="110100")
                {
                    w=w+"0";

                }
                if(p=="110101")
                {
                    w=w+"1";

                }
                if(p=="110110")
                {
                    w=w+"2";

                }
                if(p=="110111")
                {
                    w=w+"3";

                }
                if(p=="111000")
                {
                    w=w+"4";

                }
                if(p=="111001")
                {
                    w=w+"5";

                }
                if(p=="111010")
                {
                    w=w+"6";

                }
                if(p=="111011")
                {
                    w=w+"7";

                }
                if(p=="111100")
                {
                    w=w+"8";

                }
                if(p=="111101")
                {
                    w=w+"9";

                }
                if(p=="111110")
                {
                    w=w+"+";

                }
                else if(p=="111111")
                {
                    w=w+"/";

                }

        }
        binary="";
    }
    w=w+"= ";
    return w;
}
void normalize(vector<Vertex>& vertices) {
    double min_x = vertices[0].x, max_x = vertices[0].x;
    double min_y = vertices[0].y, max_y = vertices[0].y;

    for (const auto& v : vertices) {
        min_x = min(min_x, v.x);
        max_x = max(max_x, v.x);
        min_y = min(min_y, v.y);
        max_y = max(max_y, v.y);
    }

    for (auto& v : vertices) {
        v.x = (v.x - min_x) / (max_x - min_x);
        v.y = (v.y - min_y) / (max_y - min_y);
    }
}
void rand_coordinates(std::vector<Vertex>& vertices,int n)
{
    double lower_bound = 0;
   double upper_bound = 10000;
   uniform_real_distribution<double> unif(lower_bound,upper_bound);
   default_random_engine re;
   for(int i=0;i<n;i++)
   {
       double a = unif(re);
   double b = unif(re);
   //cout<<a<<" "<<b<<endl;
   Vertex point;
   point.x=a;
   point.y=b;
   point.nr=i;
   vertices[i]=point;
   }

}
bool inducesCliques(int n, bool** G, const vector<Vertex>& x) {
    bool induces_cliques = true;
    for (int i = 0; i < x.size(); ++i) {
        for (int j = i + 1; j < x.size(); ++j) {
            if (x[i].part == x[j].part && G[x[i].nr][x[j].nr] == 0) {
                induces_cliques = false;
                return induces_cliques;
            }
        }
    }

    return induces_cliques;
}
bool subinducesCliques(int n, bool** G, const std::vector<Vertex>& x,int p) {
    bool induces_cliques = true;
    for (int i = 0; i < x.size(); ++i) {
        for (int j = i + 1; j < x.size(); ++j) {
            if (x[i].part == x[j].part == p && G[x[i].nr][x[j].nr] == 0) {
                induces_cliques = false;
                return induces_cliques;
            }
        }
    }

    return induces_cliques;
}
double mean(const vector<double>& values) {
    return accumulate(values.begin(), values.end(), 0.0) / values.size();
}

double standard_deviation(const vector<double>& values, double mean_value) {
    double sum = 0;
    for (const auto& value : values) {
        sum += pow(value - mean_value, 2);
    }
    return sqrt(sum / values.size());
}

double correlation(const vector<Vertex>& vertices, double mean_x, double mean_y, double std_dev_x, double std_dev_y) {
    double sum = 0;
    for (const auto& v : vertices) {
        sum += (v.x - mean_x) * (v.y - mean_y);
    }
    return sum / (vertices.size() * std_dev_x * std_dev_y);
}

vector<int> partitionVertices(int n, vector<Vertex> &vertices,vector<int> partition,int m) {
    normalize(vertices);

    vector<double> x_values, y_values;
    for (const auto& v : vertices) {
            if(v.part==m)
        {x_values.push_back(v.x);
        y_values.push_back(v.y);}
    }

    double mean_x = mean(x_values);
    double mean_y = mean(y_values);

    double std_dev_x = standard_deviation(x_values, mean_x);
    double std_dev_y = standard_deviation(y_values, mean_y);

    if (std_dev_x == 0 && std_dev_y == 0) {
        throw runtime_error("Nie mozna rozdzielic wierzcholków ta metoda");
    }

    if (std_dev_x == 0 || std_dev_y == 0) {
        double threshold = (std_dev_x == 0) ? mean_y : mean_x;
        for (int i = 0; i < n; ++i) {
                if(vertices[i].part==m)
            //vertices[i].part = (std_dev_x == 0) ? (vertices[i].y >= threshold) : (vertices[i].x >= threshold);
            {if(std_dev_x==0)
            {
                if(vertices[i].y>=threshold)
                {
                    partition[i]=m+1;
                }
                else
                    partition[i]=m;
            }
            else
            {
                if(vertices[i].x>=threshold)
                {
                    partition[i]=m+2;
                }
                else
                    partition[i]=m+1;
            }
        }}
        return partition;
    }
    else{
        double corr = correlation(vertices, mean_x, mean_y, std_dev_x, std_dev_y);

    for (int i = 0; i < n; ++i) {
            if(vertices[i].part==m)
        {double sum_xy = vertices[i].x + vertices[i].y;
        double diff_yx = vertices[i].y - vertices[i].x;
        if (corr > 0) {
            //vertices[i].part = (sum_xy >= mean_x + mean_y);
            if(sum_xy >= mean_x + mean_y)
            {
                partition[i] =m+2;
            }
            else
                partition[i] =m+1;
        } else {
            //vertices[i].part = (diff_yx >= mean_y - mean_x);
            if(diff_yx >= mean_y - mean_x)
            {
                partition[i] =m+2;
            }
            else
                partition[i] =m+1;
        }
    }}
    return partition;
    }

}
int k1(int n, bool** G,vector<int> partition,int p)
{
    int w=0;
    vector<int> v;
    for(int i=0;i<n;i++)
    {
        if(partition[i]==p)
        {
            v.push_back(i);
        }
    }
    for(int i=0;i<v.size();i++)
    {
        for(int j=i+1;j<v.size();j++)
    {
        if(G[v[i]][v[j]])
        {
            w++;
        }
    }
    }
    return w;
}
/*int k2(int n, bool** G,vector<int> partition,int p1,int p2)
{
    int w=0;
    vector<int> v1,v2;
    for(int i=0;i<n;i++)
    {
        if(partition[i]==p1)
        {
            v1.push_back(i);
        }
        if(partition[i]==p2)
        {
            v2.push_back(i);
        }
    }
    for(int i=0;i<v1.size();i++)
    {
        for(int j=0;j<v2.size();j++)
    {
        if(G[v1[i]][v2[j]])
        {
            w++;
        }
    }
    }
    return w;
}*/
long long silnia(int n)
{
    if(n<=1)
        return 1;
    return n*silnia(n-1);
}
long long symbol(int n,int k)
{
    return silnia(n)/(silnia(k)*silnia(n-k));
}
double ocena(int n, bool** G,vector<int> partition)
{
    set<int> pom;
    for(int i=0;i<n;i++)
    {
        pom.insert(partition[i]);
    }
    double res=0;
    int ile=pom.size();
    int p=0;
    for(int i=0;i<ile;i++)
    {
        p=0;
        for(int j=0;j<n;j++)
        {
            if(partition[j]==i)
                p++;
        }
        if(p>1)
        {
            res=res+double(p)*double(k1(n,G,partition,i))/double(symbol(p,2));
        }
    }
    return res;
}
bool czy_jedynki(int* tab,int n)
{
    for(int i=0;i<n;i++)
    {
        if(tab[i]!=1)
            return false;
    }
    return true;
}
double w;
void split(int n,bool** G,vector<Vertex> &vertices,vector<int> &partition,int p,double &w)
{
    int* tab=new int [n];
    for(int i=0;i<n;i++)
    {
        tab[i]=0;
    }
    for(int i=0;i<n;i++)
    {
        tab[partition[i]]++;
    }
    double cur=ocena(n,G,partition);
    if(w<cur)
    {
        w=cur;
    }
    //while(!czy_jedynki(tab,n))
    //{

        for(int i=0;i<n;i++)
        {
            if(tab[i]>1)
            {
                //cout<<"OCENA2: "<<ocena(n,G,partitionVertices(n,vertices,partition,p))<<endl;
                if(ocena(n,G,partitionVertices(n,vertices,partition,p))<w)
                {

                    w=ocena(n,G,partitionVertices(n,vertices,partition,p));
                    partition=partitionVertices(n,vertices,partition,p);
                    split(n,G,vertices,partition,p+1,w);
                    /*cout<<"PARTYCJA"<<endl;
                    for(int i=0;i<n;i++)
                    {
                        cout<<partition[i]<<" ";
                        }
                    cout<<endl;*/
                }
            }
        }
    //}
}
void gravity(int n,bool** G,vector<Vertex> &vertices,int m)
{
    double sumx=0;
    double sumy=0;
    int pom=0;
    for(int i=0;i<n;i++)
    {
        sumx=0;
        sumy=0;
        pom=0;
        for(int j=0;j<n;j++)
        {
            if(i!=j)
            {
                if(G[i][j])
                {
                    sumx+=vertices[j].x;
                    sumy+=vertices[j].y;
                    pom++;
                }
            }
        }
        vertices[i].x=double(sumx+(double)m*(double)vertices[i].x)/double(m+pom);
        vertices[i].y=double(sumy+(double)m*(double)vertices[i].y)/double(m+pom);
    }
}
vector<int> IPG(int n,bool** G,vector<Vertex> vertices,vector<int> partition)
{
    for(int i=0;i<n;i++)
    {
        partition[i]=0;
    }
    double o=ocena(n,G,partition);
    //cout<<"OCENA1: "<<o<<endl;
    while(true)
    {
        //cout<<"K";
    for(int i=0;i<n;i++)
    {
        partition[i]=0;
    }
    double o=ocena(n,G,partition);
    /*if(inducesCliques(n,G,vertices))
    {
        return 1;
    }
        partitionVertices(n,vertices);
        for(int i=0;i<n;i++)
        {
            cout<<vertices[i].nr<<":"<<vertices[i].part<<" ";
        }
        cout<<endl;
        if(inducesCliques(n,G,vertices))
    {
        return 2;
    }
    if(subinducesCliques(n,G,vertices,0))
    {
        vector<Vertex> v2;
        for(int i=0;i<n;i++)
        {
            if(vertices[i].part==1)
                v2.push_back(vertices[i]);
        }
        return IPG(v2.size(),G,v2)+1;
    }
    if(subinducesCliques(n,G,vertices,1))
    {
        vector<Vertex> v2;
        for(int i=0;i<n;i++)
        {
            if(vertices[i].part==0)
                v2.push_back(vertices[i]);
        }
        return IPG(v2.size(),G,v2)+1;
    }
    vector<Vertex> v2;
        for(int i=0;i<n;i++)
        {
            if(vertices[i].part==1)
                v2.push_back(vertices[i]);
        }
        vector<Vertex> v3;
        for(int i=0;i<n;i++)
        {
            if(vertices[i].part==0)
                v3.push_back(vertices[i]);
        }
        return IPG(v2.size(),G,v2)+IPG(v3.size(),G,v3);*/
        vector<Vertex> before;
        for(int i=0;i<n;i++)
        {
            before.push_back(vertices[i]);
        }
        split(n,G,vertices,partition,0,o);
        gravity(n,G,vertices,2);
        normalize(vertices);
        double eps=0.2;
        bool flag=0;
        for(int i=0;i<n;i++)
        {
            if(fabs(vertices[i].x-before[i].x)>eps || fabs(vertices[i].y-before[i].y)>eps)
            {
                flag=1;
                break;
            }
        }
        //cout<<vertices[0].x<<" "<<before[0].x<<endl;
        if(!flag)
            return partition;
}

}
int num_of_cliques(vector<int> v)
{
    int w=0;
    set<int> uniq;
    for(int i=0;i<v.size();i++)
    {
        uniq.insert(v[i]);
    }
    w=uniq.size();
    return w;
}
int main() {
    unsigned short int n,e,a,b;
    fstream plik;
    string linia;
     /*vector<int> wektor={1,1,1,2,2,3,4,4,4,4,5};
    cout<<num_of_cliques(wektor)<<endl;*/
    /*plik.open("graf.txt",ios::in);
    plik>>n>>e;*/
    cin>>n;
    //getline(plik,linia);
    bool** G=new bool* [n];
    for(int i=0;i<n;i++)
    {
        G[i]=new bool [n];
    }
    for(int i=0;i<n;i++)
    {
        for(int j=0;j<n;j++)
    {
        G[i][j]=0;
    }
    }
    bool x;
    for(int i=0;i<n-1;i++)
    {
        //getline(cin,linia);
        //getline(plik,linia);
        for(int j=0;j<n-i-1;j++)
        {
            cin>>x;
                G[i][j+i+1]=x;
                G[j+i+1][i]=x;

        }

    }
    /*for(int i=0;i<n;i++)
    {
        for(int j=0;j<n;j++)
    {
        cout<<G[i][j]<<" ";
    }
    cout<<endl;
    }*/
    std::vector<Vertex> vertices(n);

    std::vector<int> partition(n);
    rand_coordinates(vertices,n);
    normalize(vertices);
    try {
        partition=IPG(n,G,vertices,partition);
        //cout<<"L";
        int ile=num_of_cliques(partition);
        vector<vector<int>> cliques;
        for(int i=0;i<ile;i++)
        {
            cliques.push_back({});
        }
        for(int i=0;i<n;i++)
        {
            cliques[partition[i]-1].push_back(i);
        }
        //cout<<"[";

        for(int i=0;i<ile;i++)
        {
            //cout<<"[";
            string in="";
            string tmp=to_string(cliques[i].size());
            //tmp=string(tmp1);
            if(cliques[i].size()<10)
            {
                tmp="0"+tmp;
            }
            in=tmp+in;
            for(int j=0;j<cliques[i].size();j++)
            {
                //cout<<cliques[i][j];
                /*if(j!=cliques[i].size()-1)
                {
                    cout<<",";
                }*/
                tmp=to_string(cliques[i][j]);
                //tmp=string(tmp1);
                while(tmp.size()<4)
                {
                    tmp="0"+tmp;
                }
                in=in+tmp;
            }
            /*cout<<"]";
            if(i!=ile-1)
            {
                cout<<",";
            }*/
            cout<<toLittleEndianBase64(in);
        }
        //cout<<"]";

        /*for(int i=0;i<n;i++)
        {
            cout<<partition[i]<<" ";
        }*/
        //cout<<endl;

    } catch (const std::exception& e) {
        std::cerr << e.what() << std::endl;
    }

    return 0;
}
